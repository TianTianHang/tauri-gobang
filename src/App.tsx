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
} from "./types/game";
import MainMenu from "./components/MainMenu";
import GameBoard from "./components/GameBoard";
import StatusBar from "./components/StatusBar";
import MenuDrawer from "./components/MenuDrawer";
import NetworkSetup from "./components/NetworkSetup";
import { useHapticFeedback } from "./hooks/useHapticFeedback";
import "./App.css";

function App() {
  const [mode, setMode] = useState<GameMode>("menu");
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [difficulty, setDifficulty] = useState<Difficulty>("medium");
  const [aiThinking, setAiThinking] = useState(false);
  const [lastMove, setLastMove] = useState<{ row: number; col: number } | null>(null);
  const [localIp, setLocalIp] = useState("");
  const [networkError, setNetworkError] = useState("");
  const [networkLoading, setNetworkLoading] = useState(false);
  const [undoRequested, setUndoRequested] = useState(false);
  const [restartRequested, setRestartRequested] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const unsubFns = useRef<UnlistenFn[]>([]);
  const haptic = useHapticFeedback();

  useEffect(() => {
    const unlistenAiMove = listen<{
      row: number;
      col: number;
      state: GameState;
    }>("ai:move_completed", (event) => {
      console.log("✅ [AI] Received AI move:", event.payload);
      setGameState(event.payload.state);
      setLastMove({ row: event.payload.row, col: event.payload.col });
      setAiThinking(false);
      const status = event.payload.state.status;
      if (status === GameStatus.BlackWins || status === GameStatus.WhiteWins) {
        haptic.win();
      }
    });

    const unlistenAiError = listen<string>("ai:move_error", (event) => {
      console.error("❌ [AI] AI error:", event.payload);
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
    invoke<string>("get_local_ip")
      .then(setLocalIp)
      .catch(() => setLocalIp(""));
  }, []);

  useEffect(() => {
    return () => {
      cleanupListeners();
    };
  }, [cleanupListeners]);

  const startNewGame = useCallback(async () => {
    const state = await invoke<GameState>("new_game");
    setGameState(state);
    setLastMove(null);
    setAiThinking(false);
    setUndoRequested(false);
    setRestartRequested(false);
  }, []);

  const handlePlayAI = useCallback(() => {
    setMode("ai");
    setNetworkError("");
    startNewGame();
  }, [startNewGame]);

  const setupNetworkListeners = useCallback(
    (newMode: GameMode) => {
      cleanupListeners();

      const unlistenPromises = [
        listen<string>("network:opponent_moved", (event) => {
          const data = JSON.parse(event.payload);
          const { row, col } = data;
          invoke<MoveResult>("make_move", { state: gameState, row, col })
            .then((result) => {
              setGameState(result.state);
              setLastMove({ row, col });
            })
            .catch(console.error);
        }),
        listen<string>("network:disconnected", () => {
          setNetworkError("对手已断开连接");
          setMode("menu");
        }),
        listen<string>("network:undo_request", () => {
          setUndoRequested(true);
        }),
        listen<string>("network:undo_accept", () => {
          if (gameState) {
            invoke<MoveResult>("undo_two_moves", { state: gameState })
              .then((result) => {
                setGameState(result.state);
                setUndoRequested(false);
              })
              .catch(console.error);
          }
        }),
        listen<string>("network:undo_reject", () => {
          setUndoRequested(false);
        }),
        listen<string>("network:restart_request", () => {
          setRestartRequested(true);
        }),
        listen<string>("network:restart_accept", () => {
          startNewGame();
          setRestartRequested(false);
        }),
        listen<string>("network:restart_reject", () => {
          setRestartRequested(false);
        }),
      ];

      Promise.all(unlistenPromises).then((fns) => {
        unsubFns.current = fns;
      });

      setMode(newMode);
    },
    [gameState, cleanupListeners, startNewGame]
  );

  const handleHostGame = useCallback(
    async (port: number) => {
      setNetworkError("");
      setNetworkLoading(true);
      try {
        await invoke<string>("network_host", { port });
        setupNetworkListeners("online_host");
        await startNewGame();
        setNetworkLoading(false);
      } catch (e) {
        setNetworkError(String(e));
        setNetworkLoading(false);
      }
    },
    [setupNetworkListeners, startNewGame]
  );

  const handleJoinGame = useCallback(
    async (ip: string, port: number) => {
      setNetworkError("");
      setNetworkLoading(true);
      try {
        await invoke<void>("network_join", { ip, port });
        setupNetworkListeners("online_client");
        await startNewGame();
        setNetworkLoading(false);
      } catch (e) {
        setNetworkError(String(e));
        setNetworkLoading(false);
      }
    },
    [setupNetworkListeners, startNewGame]
  );

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
          await invoke<void>("network_send_move", { row, col });
        }

        if (mode === "ai" && result.ai_thinking) {
          setAiThinking(true);
          await invoke<void>("ai_move_start", { state: result.state, difficulty });
        }
      } catch (e) {
        console.error("Move error:", e);
      }
    },
    [gameState, aiThinking, isOnline, myColor, mode, difficulty]
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
    if (isOnline) {
      try {
        await invoke<void>("network_send_restart_request");
      } catch {
        startNewGame();
      }
    } else {
      startNewGame();
    }
  }, [isOnline, startNewGame]);

  const handleBackToMenu = useCallback(async () => {
    if (isOnline) {
      try {
        await invoke<void>("network_disconnect");
      } catch {
        // ignore
      }
    }
    cleanupListeners();
    setMode("menu");
    setGameState(null);
    setLastMove(null);
    setAiThinking(false);
    setUndoRequested(false);
    setRestartRequested(false);
    setMenuOpen(false);
  }, [isOnline, cleanupListeners]);

  const handleUndoRequest = useCallback(async () => {
    try {
      await invoke<void>("network_send_undo_request");
    } catch (e) {
      console.error(e);
    }
  }, []);

  const handleRestartRequest = useCallback(async () => {
    try {
      await invoke<void>("network_send_restart_request");
    } catch (e) {
      console.error(e);
    }
  }, []);

  const handleAcceptUndo = useCallback(async () => {
    try {
      await invoke<void>("network_send_undo_accept");
      if (gameState) {
        const result = await invoke<MoveResult>("undo_two_moves", { state: gameState });
        setGameState(result.state);
        const history = result.state.history;
        if (history.length > 0) {
          const last = history[history.length - 1];
          setLastMove({ row: last.row, col: last.col });
        } else {
          setLastMove(null);
        }
      }
      setUndoRequested(false);
    } catch (e) {
      console.error(e);
    }
  }, [gameState]);

  const handleRejectUndo = useCallback(async () => {
    try {
      await invoke<void>("network_send_undo_reject");
    } catch (e) {
      console.error(e);
    }
    setUndoRequested(false);
  }, []);

  const handleAcceptRestart = useCallback(async () => {
    try {
      await invoke<void>("network_send_restart_accept");
    } catch (e) {
      console.error(e);
    }
    startNewGame();
    setRestartRequested(false);
  }, [startNewGame]);

  const handleRejectRestart = useCallback(async () => {
    try {
      await invoke<void>("network_send_restart_request");
    } catch (e) {
      console.error(e);
    }
    setRestartRequested(false);
  }, []);

  if (mode === "menu") {
    return (
      <MainMenu
        onPlayAI={handlePlayAI}
        onHostGame={() => setMode("host_setup")}
        onJoinGame={() => setMode("join_setup")}
      />
    );
  }

  if (mode === "host_setup") {
    return (
      <div className="setup-page">
        <NetworkSetup
          mode="host"
          onHost={handleHostGame}
          onJoin={() => {}}
          onCancel={() => setMode("menu")}
          localIp={localIp}
          loading={networkLoading}
          error={networkError}
        />
      </div>
    );
  }

  if (mode === "join_setup") {
    return (
      <div className="setup-page">
        <NetworkSetup
          mode="join"
          onHost={() => {}}
          onJoin={handleJoinGame}
          onCancel={() => setMode("menu")}
          localIp={localIp}
          loading={networkLoading}
          error={networkError}
        />
      </div>
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
        myColor={myColor}
        onUndoRequest={isOnline ? handleUndoRequest : undefined}
        onRestartRequest={isOnline ? handleRestartRequest : undefined}
        undoRequested={undoRequested}
        restartRequested={restartRequested}
        onAcceptUndo={handleAcceptUndo}
        onRejectUndo={handleRejectUndo}
        onAcceptRestart={handleAcceptRestart}
        onRejectRestart={handleRejectRestart}
      />
    </div>
  );
}

export default App;
