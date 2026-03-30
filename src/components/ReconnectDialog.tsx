import { useEffect, useState, useRef } from "react";
import "./ReconnectDialog.css";

interface ReconnectDialogProps {
  visible: boolean;
  timeoutSeconds: number;
  opponentName: string;
  onReconnectSuccess: () => void;
  onTimeout: () => void;
  reconnectWs: () => WebSocket | null;
}

function ReconnectDialog({
  visible,
  timeoutSeconds,
  opponentName,
  onReconnectSuccess,
  onTimeout,
  reconnectWs,
}: ReconnectDialogProps) {
  const [remaining, setRemaining] = useState(timeoutSeconds);
  const [attempt, setAttempt] = useState(0);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    if (!visible) {
      setRemaining(timeoutSeconds);
      setAttempt(0);
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      return;
    }

    const startTime = Date.now();
    intervalRef.current = setInterval(() => {
      const elapsed = Math.floor((Date.now() - startTime) / 1000);
      const left = timeoutSeconds - elapsed;
      if (left <= 0) {
        if (intervalRef.current) clearInterval(intervalRef.current);
        onTimeout();
        return;
      }
      setRemaining(left);

      const newAttempt = Math.floor(elapsed / 5) + 1;
      if (newAttempt > attempt && newAttempt <= 6) {
        setAttempt(newAttempt);
        const ws = reconnectWs();
        if (ws) {
          if (intervalRef.current) clearInterval(intervalRef.current);
          onReconnectSuccess();
        }
      }
    }, 1000);

    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, [visible, timeoutSeconds, onTimeout, onReconnectSuccess, reconnectWs, attempt]);

  if (!visible) return null;

  return (
    <div className="reconnect-overlay">
      <div className="reconnect-dialog">
        <div className="reconnect-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
            <path d="M1 1v22h22" />
            <path d="M17 17l-4-4 4-4" />
            <path d="M13 13h6" />
          </svg>
        </div>
        <h3 className="reconnect-title">对手已断开连接</h3>
        <p className="reconnect-opponent">{opponentName}</p>
        <div className="reconnect-countdown">
          <div className="countdown-circle">
            <span className="countdown-number">{remaining}</span>
          </div>
          <p className="countdown-label">秒后超时</p>
        </div>
        <p className="reconnect-status">正在尝试重连... ({attempt}/6)</p>
      </div>
    </div>
  );
}

export default ReconnectDialog;
