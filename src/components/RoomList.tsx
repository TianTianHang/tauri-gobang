import { useState, useEffect, useCallback } from "react";
import type { RoomListEntry } from "../types/game";
import { getRooms, createRoom } from "../api";
import { StatusDotIcon, UserIcon, ClockIcon, PlusIcon, RefreshIcon } from "./Icons";
import "./RoomList.css";

interface RoomListProps {
  token: string;
  username: string;
  onCreateRoom: (roomId: string, roomName: string) => void;
  onJoinRoom: (roomId: string) => void;
  onLogout: () => void;
}

function getRoomStatus(room: RoomListEntry): { color: string; label: string } {
  const playerCount = room.player_count ?? 1;
  const status = room.status ?? "waiting";
  if (playerCount >= 2) {
    return { color: "var(--room-full)", label: "满员" };
  }
  if (status === "waiting") {
    return { color: "var(--room-available)", label: "可加入" };
  }
  return { color: "var(--room-waiting)", label: "等待中" };
}

function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diff = Math.floor((now - timestamp * 1000) / 1000);
  if (diff < 0) return "刚刚";
  if (diff < 60) return "刚刚";
  if (diff < 3600) {
    const minutes = Math.floor(diff / 60);
    return `${minutes}分钟前`;
  }
  if (diff < 86400) {
    const hours = Math.floor(diff / 3600);
    return `${hours}小时前`;
  }
  return new Date(timestamp * 1000).toLocaleString("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function RoomList({ token, username, onCreateRoom, onJoinRoom, onLogout }: RoomListProps) {
  const [rooms, setRooms] = useState<RoomListEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [showCreate, setShowCreate] = useState(false);
  const [roomName, setRoomName] = useState("");
  const [creating, setCreating] = useState(false);

  const fetchRooms = useCallback(async () => {
    try {
      const res = await getRooms(token);
      setRooms(res.rooms);
      setError("");
    } catch (e) {
      setError(e instanceof Error ? e.message : "获取房间列表失败");
    } finally {
      setLoading(false);
    }
  }, [token]);

  useEffect(() => {
    fetchRooms();
    const interval = setInterval(fetchRooms, 5000);
    return () => clearInterval(interval);
  }, [fetchRooms]);

  async function handleCreateRoom(e: React.FormEvent) {
    e.preventDefault();
    if (!roomName.trim()) return;
    setCreating(true);
    try {
      const res = await createRoom(token, roomName.trim());
      onCreateRoom(res.room_id, res.room_name);
    } catch (e) {
      setError(e instanceof Error ? e.message : "创建房间失败");
    } finally {
      setCreating(false);
    }
  }

  return (
    <div className="setup-page">
      <div className="room-list">
        <div className="lobby-header">
          <h2>游戏大厅</h2>
          <span className="lobby-user">欢迎, {username}</span>
        </div>

        <div className="lobby-actions">
          <button className="lobby-btn-primary" onClick={() => setShowCreate(true)}>
            <PlusIcon className="lobby-btn-icon" />
            创建房间
          </button>
          <div className="lobby-actions-secondary">
            <button className="btn-secondary" onClick={fetchRooms} disabled={loading}>
              <RefreshIcon className="lobby-btn-icon" />
              刷新
            </button>
            <button className="btn-secondary" onClick={onLogout}>
              退出登录
            </button>
          </div>
        </div>

        {showCreate && (
          <div className="create-room-dialog">
            <form onSubmit={handleCreateRoom}>
              <div className="form-group">
                <label>房间名称</label>
                <input
                  type="text"
                  value={roomName}
                  onChange={(e) => setRoomName(e.target.value)}
                  placeholder="来一局五子棋吧"
                  disabled={creating}
                  autoFocus
                  maxLength={50}
                />
                <span className="helper-text">💡 给房间起个有趣的名字</span>
              </div>
              <div className="setup-buttons">
                <button type="submit" className="btn-primary" disabled={creating || !roomName.trim()}>
                  {creating ? "创建中..." : "确认创建"}
                </button>
                <button type="button" className="btn-cancel" onClick={() => { setShowCreate(false); setRoomName(""); }}>
                  取消
                </button>
              </div>
            </form>
          </div>
        )}

        {error && <p className="setup-error">{error}</p>}

        <div className="rooms-container">
          {loading && rooms.length === 0 ? (
            <p className="rooms-empty">加载中...</p>
          ) : rooms.length === 0 ? (
            <div className="rooms-empty">
              <span className="rooms-empty-icon">🏠</span>
              <p className="rooms-empty-title">大厅空空如也...</p>
              <p className="rooms-empty-subtitle">创建一个房间等待朋友吧</p>
            </div>
          ) : (
            <div className="rooms-list">
              {rooms.map((room) => {
                const status = getRoomStatus(room);
                return (
                  <div key={room.id} className="room-card">
                    <div className="room-info">
                      <span className="room-name">
                        <StatusDotIcon color={status.color} />
                        {room.name}
                      </span>
                      <span className="room-meta">
                        <span className="player-count" style={{ color: status.color }}>
                          {room.player_count ?? 1}/2
                        </span>
                        <span className="room-host">
                          <UserIcon className="room-icon" />
                          {room.host_username}
                        </span>
                      </span>
                      <span className="room-time">
                        <ClockIcon className="room-icon room-icon-sm" />
                        {formatRelativeTime(room.created_at)}
                      </span>
                    </div>
                    <button className="btn-join" onClick={() => onJoinRoom(room.id)}>
                      加入
                    </button>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default RoomList;
