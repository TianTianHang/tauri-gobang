import { useState } from "react";
import "./NetworkSetup.css";

interface NetworkSetupProps {
  mode: "host" | "join";
  onHost: (port: number) => void;
  onJoin: (ip: string, port: number) => void;
  onCancel: () => void;
  localIp: string;
  loading: boolean;
  error?: string;
}

function NetworkSetup({ mode, onHost, onJoin, onCancel, localIp, loading, error }: NetworkSetupProps) {
  const [ip, setIp] = useState("");
  const [port, setPort] = useState("5555");

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const p = parseInt(port, 10);
    if (isNaN(p) || p < 1 || p > 65535) return;

    if (mode === "host") {
      onHost(p);
    } else {
      if (!ip.trim()) return;
      onJoin(ip.trim(), p);
    }
  }

  return (
    <div className="network-setup">
      <h2>{mode === "host" ? "创建房间" : "加入房间"}</h2>

      {mode === "host" && (
        <div className="setup-info">
          <p>你的 IP 地址:</p>
          <div className="ip-display">{localIp || "加载中..."}</div>
          <p className="setup-hint">
            将 IP 地址和端口号告诉对手，等待对方加入
          </p>
        </div>
      )}

      <form onSubmit={handleSubmit}>
        {mode === "join" && (
          <div className="form-group">
            <label>服务器 IP 地址</label>
            <input
              type="text"
              value={ip}
              onChange={(e) => setIp(e.target.value)}
              placeholder="例如: 192.168.1.100"
              disabled={loading}
              autoFocus
            />
          </div>
        )}

        <div className="form-group">
          <label>端口号</label>
          <input
            type="number"
            value={port}
            onChange={(e) => setPort(e.target.value)}
            placeholder="5555"
            min="1"
            max="65535"
            disabled={loading}
          />
        </div>

        {error && <p className="setup-error">{error}</p>}

        <div className="setup-buttons">
          <button type="submit" className="btn-primary" disabled={loading}>
            {loading
              ? mode === "host"
                ? "等待连接..."
                : "连接中..."
              : mode === "host"
                ? "创建房间"
                : "加入房间"}
          </button>
          <button type="button" className="btn-cancel" onClick={onCancel} disabled={loading}>
            取消
          </button>
        </div>
      </form>
    </div>
  );
}

export default NetworkSetup;
