import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import {
  GameState,
  GameMode,
  Difficulty,
  Cell,
  GameStatus,
  MoveResult,
  ServerMessage,
  NetworkMessage,
} from "./types/game";
import { getGameWsUrl, joinRoom } from "./api";
import MainMenu from "./components/MainMenu";
import GameBoard from "./components/GameBoard";
import StatusBar from "./components/StatusBar";
import MenuDrawer from "./components/MenuDrawer";
import LoginScreen from "./components/LoginScreen";
import RoomList from "./components/RoomList";
import WaitingRoom from "./components/WaitingRoom";
import ReconnectDialog from "./components/ReconnectDialog";
import GameResultModal from "./components/GameResultModal";
import { useHapticFeedback } from "./hooks/useHapticFeedback";
import "./App.css";

function App() {
  const [mode, setMode] = useState<GameMode>("menu");
  const [gameState, setGameState] = useState<GameState | null>(null);
  const gameStateRef = useRef(gameState);
  const [difficulty, setDifficulty] = useState<Difficulty>("medium");
  const [aiThinking, setAiThinking] = useState(false);
  const [lastMove, setLastMove] = useState<{ row: number; col: number } | null>(null);
  const [restartRequested, setRestartRequested] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [gameDuration, setGameDuration] = useState<number>(0);
  const gameStartTimeRef = useRef<number>(Date.now());
  const unsubFns = useRef<UnlistenFn[]>([]);
  const haptic = useHapticFeedback();

  // Server auth state
  const [token, setToken] = useState<string>(() => localStorage.getItem("gobang_token") || "");
  const [username, setUsername] = useState<string>(() => localStorage.getItem("gobang_username") || "");

  // Online game state
  const [opponentName, setOpponentName] = useState("");
  const [currentRoomId, setCurrentRoomId] = useState("");
  const [currentRoomName, setCurrentRoomName] = useState("");
  const [isHost, setIsHost] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);
  const [showReconnect, setShowReconnect] = useState(false);
  const [reconnectTimeout, setReconnectTimeout] = useState(30);

  useEffect(() => {
    gameStateRef.current = gameState;
  }, [gameState]);

  useEffect(() => {
    if (gameState && gameState.status !== GameStatus.Playing) {
      setGameDuration(Math.floor((Date.now() - gameStartTimeRef.current) / 1000));
    }
  }, [gameState?.status]);

  // AI event listeners
  useEffect(() => {
    const unlistenAiMove = listen<{
      row: number;
      col: number;
      state: GameState;
    }>("ai:move_completed", (event) => {
      setGameState(event.payload.state);
      setLastMove({ row: event.payload.row, col: event.payload.col });
      setAiThinking(false);
      const status = event.payload.state.status;
      if (status === GameStatus.BlackWins || status === GameStatus.WhiteWins) {
        haptic.win();
      }
    });

    const unlistenAiError = listen<string>("ai:move_error", (event) => {
      console.error("AI error:", event.payload);
      setAiThinking(false);
    });

    return () => {
      unlistenAiMove.then((fn) => fn());
      unlistenAiError.then((fn) => fn());
    };
  }, [haptic]);

  const isOnline = mode === "online_host" || mode === "online_client";
  const myColor = mode === "online_host" ? Cell.Black : mode === "online_client" ? Cell.White : undefined;
  const isMyTurn = !isOnline || gameState?.current_player === myColor;

  const cleanupListeners = useCallback(async () => {
    for (const fn of unsubFns.current) {
      fn();
    }
    unsubFns.current = [];
  }, []);

  useEffect(() => {
    return () => {
      cleanupListeners();
    };
  }, [cleanupListeners]);

  // WebSocket management
  const closeWs = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.onopen = null;
      wsRef.current.onclose = null;
      wsRef.current.onmessage = null;
      wsRef.current.onerror = null;
      if (wsRef.current.readyState === WebSocket.OPEN) {
        wsRef.current.close();
      }
      wsRef.current = null;
    }
  }, []);

  const createWsMessageHandler = useCallback((
    role: "host" | "client" | null,
  ) => {
    return (event: MessageEvent) => {
      try {
        const msg: ServerMessage = JSON.parse(event.data);
        switch (msg.type) {
          case "game_start": {
            if (!role) break;
            const black = msg.black_player!;
            const white = msg.white_player!;
            const oppName = role === "host" ? white : black;
            setOpponentName(oppName);
            if (!gameStateRef.current) {
              invoke<GameState>("new_game").then((state) => {
                setGameState(state);
                setLastMove(null);
                setRestartRequested(false);
                gameStartTimeRef.current = Date.now();
                setGameDuration(0);
                setMode(role === "host" ? "online_host" : "online_client");
              });
            } else {
              setMode(role === "host" ? "online_host" : "online_client");
            }
            break;
          }
          case "opponent_joined": {
            if (msg.username) {
              setOpponentName(msg.username);
            }
            break;
          }
          case "opponent_disconnected": {
            setShowReconnect(true);
            setReconnectTimeout(msg.timeout_seconds || 30);
            break;
          }
          case "player_reconnected": {
            setShowReconnect(false);
            break;
          }
          case "game_ended": {
            setShowReconnect(false);
            if (gameStateRef.current) {
              let endStatus: GameStatus;
              if (msg.reason === "draw") {
                endStatus = GameStatus.Draw;
              } else {
                endStatus = GameStatus.BlackWins;
              }
              setGameState({
                ...gameStateRef.current,
                status: endStatus,
              });
            }
            break;
          }
          case "game_state_sync": {
            if (msg.moves && msg.moves.length > 0) {
              (async () => {
                try {
                  let state = await invoke<GameState>("new_game");
                  for (const move of msg.moves!) {
                    const result = await invoke<MoveResult>("make_move", {
                      state, row: move.row, col: move.col,
                    });
                    state = result.state;
                  }
                  setGameState(state);
                  const last = state.history[state.history.length - 1];
                  setLastMove(last ? { row: last.row, col: last.col } : null);
                } catch (e) {
                  console.error("[WS] Game state sync failed:", e);
                }
              })();
            }
            break;
          }
          default: {
            const netMsg = msg as unknown as NetworkMessage;
            if (netMsg.type === "move" && netMsg.row !== undefined && netMsg.col !== undefined) {
              const currentState = gameStateRef.current;
              if (currentState && currentState.status === GameStatus.Playing) {
                invoke<MoveResult>("make_move", { state: currentState, row: netMsg.row, col: netMsg.col })
                  .then((result) => {
                    setGameState(result.state);
                    setLastMove({ row: netMsg.row!, col: netMsg.col! });
                  })
                  .catch(console.error);
              }
            } else if (netMsg.type === "restart_request") {
              setRestartRequested(true);
            } else if (netMsg.type === "restart_accept") {
              invoke<GameState>("new_game").then((state) => {
                setGameState(state);
                setLastMove(null);
                setRestartRequested(false);
                gameStartTimeRef.current = Date.now();
              });
            } else if (netMsg.type === "restart_reject") {
              setRestartRequested(false);
            }
            break;
          }
        }
      } catch (e) {
        console.error("[WS] Failed to parse message:", event.data, e);
      }
    };
  }, [username]);

  const connectWs = useCallback(
    (roomId: string, tok: string, role: "host" | "client"): WebSocket => {
      closeWs();
      const wsUrl = getGameWsUrl(roomId, tok);
      const ws = new WebSocket(wsUrl);

      ws.onopen = () => {
        console.log("[WS] Connected to", wsUrl);
      };

      ws.onmessage = createWsMessageHandler(role);

      ws.onclose = () => {
        console.log("[WS] Connection closed");
      };

      ws.onerror = (e) => {
        console.error("[WS] Error:", e);
      };

      wsRef.current = ws;
      return ws;
    },
    [closeWs, createWsMessageHandler]
  );

  const sendWs = useCallback((msg: NetworkMessage) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
    }
  }, []);

  // Auth handlers
  const handleLoginSuccess = useCallback(
    (tok: string, uname: string) => {
      setToken(tok);
      setUsername(uname);
      localStorage.setItem("gobang_token", tok);
      localStorage.setItem("gobang_username", uname);
      setMode("lobby");
    },
    []
  );

  const handleLogout = useCallback(() => {
    closeWs();
    setToken("");
    setUsername("");
    localStorage.removeItem("gobang_token");
    localStorage.removeItem("gobang_username");
    setMode("menu");
  }, [closeWs]);

  // Room handlers
  const handleCreateRoom = useCallback(
    (roomId: string, roomName: string) => {
      setCurrentRoomId(roomId);
      setCurrentRoomName(roomName);
      setIsHost(true);
      connectWs(roomId, token, "host");
      setMode("waiting");
    },
    [connectWs, token]
  );

  const handleJoinRoom = useCallback(
    async (roomId: string) => {
      try {
        await joinRoom(token, roomId);
        setCurrentRoomId(roomId);
        setIsHost(false);
        connectWs(roomId, token, "client");
        setMode("waiting");
      } catch (e) {
        alert(e instanceof Error ? e.message : "加入房间失败");
      }
    },
    [connectWs, token]
  );

  const handleCancelWaiting = useCallback(() => {
    closeWs();
    setCurrentRoomId("");
    setCurrentRoomName("");
    setIsHost(false);
    setMode("lobby");
  }, [closeWs]);

  // Reconnect
  const reconnectWs = useCallback(() => {
    if (!currentRoomId || !token) return Promise.reject("Missing room/token");

    closeWs();

    return new Promise<WebSocket>((resolve, reject) => {
      try {
        const wsUrl = getGameWsUrl(currentRoomId, token);
        const ws = new WebSocket(wsUrl);

        const timeoutId = setTimeout(() => {
          reject("Reconnection timeout");
          if (ws.readyState === WebSocket.CONNECTING) {
            ws.close();
          }
        }, 5000);

        ws.onopen = () => {
          console.log("[WS] Reconnected successfully to", wsUrl);
          clearTimeout(timeoutId);
          setShowReconnect(false);
          wsRef.current = ws;
          resolve(ws);
        };

        ws.onmessage = createWsMessageHandler(null);

        ws.onerror = (event) => {
          console.error("[WS] Reconnection error", event);
          clearTimeout(timeoutId);
          reject("Connection failed");
        };

      } catch (error) {
        reject(error);
      }
    });
  }, [currentRoomId, token, closeWs, createWsMessageHandler]);

  const handleReconnectTimeout = useCallback(() => {
    setShowReconnect(false);
    closeWs();
    setMode("menu");
    setGameState(null);
    setCurrentRoomId("");
    setOpponentName("");
  }, [closeWs]);

  // Game handlers
  const startNewGame = useCallback(async () => {
    const state = await invoke<GameState>("new_game");
    setGameState(state);
    setLastMove(null);
    setAiThinking(false);
    setRestartRequested(false);
    gameStartTimeRef.current = Date.now();
    setGameDuration(0);
  }, []);

  const handlePlayAI = useCallback(() => {
    setMode("ai");
    startNewGame();
  }, [startNewGame]);

  const handleLocalPlay = useCallback(() => {
    setMode("local_pvp");
    startNewGame();
  }, [startNewGame]);

  const handleOnlinePlay = useCallback(() => {
    if (token) {
      setMode("lobby");
    } else {
      setMode("login");
    }
  }, [token]);

  const handleCellClick = useCallback(
    async (row: number, col: number) => {
      if (!gameState || gameState.status !== GameStatus.Playing) return;
      if (aiThinking) return;
      if (isOnline && gameState.current_player !== myColor) return;

      try {
        const result = await invoke<MoveResult>("make_move", { state: gameState, row, col });
        setGameState(result.state);
        setLastMove({ row, col });

        if (isOnline) {
          sendWs({ type: "move", row, col });
        }

        if (mode === "ai" && result.ai_thinking) {
          setAiThinking(true);
          await invoke<void>("ai_move_start", { state: result.state, difficulty });
        }
      } catch (e) {
        console.error("Move error:", e);
      }
    },
    [gameState, aiThinking, isOnline, myColor, mode, difficulty, sendWs]
  );

  const handlePiecePlaced = useCallback(() => {
    haptic.place();
  }, [haptic]);

  const handleMenuOpen = useCallback(() => {
    haptic.menuOpen();
    setMenuOpen(true);
  }, [haptic]);

  const handleMenuClose = useCallback(() => {
    setMenuOpen(false);
  }, []);

  const handleUndo = useCallback(async () => {
    if (!gameState) return;
    try {
      const result = await invoke<MoveResult>("undo_two_moves", { state: gameState });
      setGameState(result.state);
      const history = result.state.history;
      if (history.length > 0) {
        const last = history[history.length - 1];
        setLastMove({ row: last.row, col: last.col });
      } else {
        setLastMove(null);
      }
    } catch (e) {
      console.error("Undo error:", e);
    }
  }, [gameState]);

  const handleNewGame = useCallback(async () => {
    startNewGame();
  }, [startNewGame]);

  const handleBackToMenu = useCallback(async () => {
    closeWs();
    cleanupListeners();
    setMode("menu");
    setGameState(null);
    setLastMove(null);
    setAiThinking(false);
    setRestartRequested(false);
    setMenuOpen(false);
    setCurrentRoomId("");
    setOpponentName("");
    setIsHost(false);
    setShowReconnect(false);
  }, [closeWs, cleanupListeners]);

  const handleRestartRequest = useCallback(() => {
    sendWs({ type: "restart_request" });
  }, [sendWs]);

  const handleAcceptRestart = useCallback(async () => {
    sendWs({ type: "restart_accept" });
    setRestartRequested(false);
  }, [sendWs]);

  const handleRejectRestart = useCallback(() => {
    sendWs({ type: "restart_reject" });
    setRestartRequested(false);
  }, [sendWs]);

  // ===== ROUTING =====

  if (mode === "menu") {
    return <MainMenu onPlayAI={handlePlayAI} onLocalPlay={handleLocalPlay} onOnlinePlay={handleOnlinePlay} />;
  }

  if (mode === "login") {
    return (
      <LoginScreen
        onLoginSuccess={handleLoginSuccess}
        onBack={() => setMode("menu")}
      />
    );
  }

  if (mode === "lobby") {
    return (
      <RoomList
        token={token}
        username={username}
        onCreateRoom={handleCreateRoom}
        onJoinRoom={handleJoinRoom}
        onLogout={handleLogout}
      />
    );
  }

  if (mode === "waiting") {
    return (
      <WaitingRoom
        roomId={currentRoomId}
        roomName={currentRoomName}
        isHost={isHost}
        onCancel={handleCancelWaiting}
      />
    );
  }

  if (!gameState) return null;

  return (
    <div className="game-page">
      <StatusBar
        gameState={gameState}
        aiThinking={aiThinking}
        mode={mode}
        myColor={myColor}
        onMenuOpen={handleMenuOpen}
        menuOpen={menuOpen}
        opponentName={opponentName}
      />
      <GameBoard
        gameState={gameState}
        onCellClick={handleCellClick}
        disabled={aiThinking || (isOnline && !isMyTurn)}
        lastMove={lastMove}
        onPiecePlaced={handlePiecePlaced}
      />
      <MenuDrawer
        isOpen={menuOpen}
        onClose={handleMenuClose}
        gameState={gameState}
        difficulty={difficulty}
        onDifficultyChange={setDifficulty}
        onUndo={handleUndo}
        onNewGame={handleNewGame}
        onBackToMenu={handleBackToMenu}
        mode={mode}
        aiThinking={aiThinking}
        onRestartRequest={isOnline ? handleRestartRequest : undefined}
        restartRequested={restartRequested}
        onAcceptRestart={handleAcceptRestart}
        onRejectRestart={handleRejectRestart}
      />
      {gameState.status !== GameStatus.Playing && (
        <GameResultModal
          gameState={gameState}
          mode={mode}
          myColor={myColor}
          gameDuration={gameDuration}
          onNewGame={handleNewGame}
          onBackToMenu={handleBackToMenu}
        />
      )}
      {showReconnect && (
        <ReconnectDialog
          visible={showReconnect}
          timeoutSeconds={reconnectTimeout}
          opponentName={opponentName}
          onReconnectSuccess={() => setShowReconnect(false)}
          onTimeout={handleReconnectTimeout}
          reconnectWs={reconnectWs}
        />
      )}
    </div>
  );
}

export default App;
