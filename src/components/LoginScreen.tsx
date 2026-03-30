import { useState } from "react";
import "./LoginScreen.css";

interface LoginScreenProps {
  onLoginSuccess: (token: string, username: string) => void;
  onBack: () => void;
}

function LoginScreen({ onLoginSuccess, onBack }: LoginScreenProps) {
  const [isRegister, setIsRegister] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError("");

    if (username.trim().length < 3 || username.trim().length > 20) {
      setError("用户名长度需在 3-20 个字符之间");
      return;
    }

    if (password.length < 6) {
      setError("密码至少需要 6 个字符");
      return;
    }

    if (isRegister && password !== confirmPassword) {
      setError("两次输入的密码不一致");
      return;
    }

    setLoading(true);
    try {
      const { register, login } = await import("../api");
      if (isRegister) {
        await register(username.trim(), password);
      }
      const res = await login(username.trim(), password);
      onLoginSuccess(res.token, res.username);
    } catch (e) {
      setError(e instanceof Error ? e.message : "操作失败");
    } finally {
      setLoading(false);
    }
  }

  function toggleMode() {
    setIsRegister(!isRegister);
    setError("");
    setConfirmPassword("");
  }

  return (
    <div className="setup-page">
      <div className="login-screen">
        <h2>{isRegister ? "注册账号" : "登录"}</h2>

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label>用户名</label>
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="3-20 个字符"
              disabled={loading}
              autoFocus
              autoComplete="username"
            />
          </div>

          <div className="form-group">
            <label>密码</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder={isRegister ? "至少 6 个字符" : ""}
              disabled={loading}
              autoComplete={isRegister ? "new-password" : "current-password"}
            />
          </div>

          {isRegister && (
            <div className="form-group">
              <label>确认密码</label>
              <input
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                placeholder="再次输入密码"
                disabled={loading}
                autoComplete="new-password"
              />
            </div>
          )}

          {error && <p className="setup-error">{error}</p>}

          <div className="setup-buttons">
            <button type="submit" className="btn-primary" disabled={loading}>
              {loading
                ? "处理中..."
                : isRegister
                  ? "注册"
                  : "登录"}
            </button>
            <button type="button" className="btn-cancel" onClick={onBack} disabled={loading}>
              返回
            </button>
          </div>

          <button
            type="button"
            className="toggle-mode-btn"
            onClick={toggleMode}
            disabled={loading}
          >
            {isRegister ? "已有账号？去登录" : "没有账号？去注册"}
          </button>
        </form>
      </div>
    </div>
  );
}

export default LoginScreen;
