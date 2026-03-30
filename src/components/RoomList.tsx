import { useState, useEffect, useCallback } from "react";
import type { RoomListEntry } from "../types/game";
import { getRooms, createRoom } from "../api";
import "./RoomList.css";

interface RoomListProps {
  token: string;
  username: string;
  onCreateRoom: (roomId: string, roomName: string) => void;
  onJoinRoom: (roomId: string) => void;
  onLogout: () => void;
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

  function formatTime(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    return date.toLocaleTimeString("zh-CN", { hour: "2-digit", minute: "2-digit" });
  }

  return (
    <div className="setup-page">
      <div className="room-list">
        <div className="lobby-header">
          <h2>游戏大厅</h2>
          <span className="lobby-user">欢迎, {username}</span>
        </div>

        <div className="lobby-actions">
          <button className="btn-primary" onClick={() => setShowCreate(true)}>
            创建房间
          </button>
          <button className="btn-refresh" onClick={fetchRooms} disabled={loading}>
            刷新
          </button>
          <button className="btn-cancel" onClick={onLogout}>
            退出登录
          </button>
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
                  placeholder="给你的房间起个名字"
                  disabled={creating}
                  autoFocus
                  maxLength={50}
                />
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
            <p className="rooms-empty">暂无可用房间，创建一个吧</p>
          ) : (
            <div className="rooms-list">
              {rooms.map((room) => (
                <div key={room.id} className="room-card">
                  <div className="room-info">
                    <span className="room-name">{room.name}</span>
                    <span className="room-host">房主: {room.host_username}</span>
                    <span className="room-time">{formatTime(room.created_at)}</span>
                  </div>
                  <button className="btn-join" onClick={() => onJoinRoom(room.id)}>
                    加入
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default RoomList;
