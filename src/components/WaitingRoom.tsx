import { useState } from "react";
import "./WaitingRoom.css";

interface WaitingRoomProps {
  roomId: string;
  roomName: string;
  isHost: boolean;
  onCancel: () => void;
}

function WaitingRoom({ roomId, roomName, onCancel }: WaitingRoomProps) {
  const [copied, setCopied] = useState(false);

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(roomId);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // ignore
    }
  }

  return (
    <div className="setup-page">
      <div className="waiting-room">
        <h2>等待对手</h2>

        <div className="waiting-info">
          <div className="waiting-room-name">
            <span className="info-label">房间名称</span>
            <span className="info-value">{roomName}</span>
          </div>
          <div className="waiting-room-id">
            <span className="info-label">房间 ID</span>
            <div className="room-id-row">
              <span className="info-value room-id-text">{roomId.slice(0, 8)}</span>
              <button className="btn-copy" onClick={handleCopy}>
                {copied ? "已复制" : "复制"}
              </button>
            </div>
          </div>
        </div>

        <div className="waiting-animation">
          <div className="waiting-dots">
            <span></span>
            <span></span>
            <span></span>
          </div>
          <p className="waiting-text">等待对手加入...</p>
        </div>

        <button className="btn-cancel waiting-cancel" onClick={onCancel}>
          取消并返回大厅
        </button>
      </div>
    </div>
  );
}

export default WaitingRoom;
