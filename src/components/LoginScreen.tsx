import { useState } from "react";
import {
  BlackStoneIcon,
  UserIcon,
  LockIcon,
  EyeIcon,
  EyeOffIcon,
  ExclamationCircleIcon,
} from "./Icons";
import "./LoginScreen.css";

interface LoginScreenProps {
  onLoginSuccess: (token: string, username: string) => void;
  onBack: () => void;
}

interface FieldErrors {
  username?: string;
  password?: string;
  confirmPassword?: string;
}

function LoginScreen({ onLoginSuccess, onBack }: LoginScreenProps) {
  const [isRegister, setIsRegister] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [errors, setErrors] = useState<FieldErrors>({});
  const [loading, setLoading] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);

  function clearError(field: keyof FieldErrors) {
    setErrors((prev) => {
      if (!prev[field]) return prev;
      const next = { ...prev };
      delete next[field];
      return next;
    });
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const newErrors: FieldErrors = {};

    if (username.trim().length < 3 || username.trim().length > 20) {
      newErrors.username = "用户名长度需在 3-20 个字符之间";
    }

    if (password.length < 6) {
      newErrors.password = "密码至少需要 6 个字符";
    }

    if (isRegister && password !== confirmPassword) {
      newErrors.confirmPassword = "两次输入的密码不一致";
    }

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors);
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
      setErrors({ password: e instanceof Error ? e.message : "操作失败" });
    } finally {
      setLoading(false);
    }
  }

  function toggleMode() {
    setIsRegister(!isRegister);
    setErrors({});
    setConfirmPassword("");
    setShowPassword(false);
    setShowConfirmPassword(false);
  }

  return (
    <div className="setup-page">
      <div className="login-screen">
        <div className="login-brand">
          <BlackStoneIcon className="brand-icon" />
          <h2 className="login-title">
            {isRegister ? "创建五子棋账号" : "欢迎回到五子棋"}
          </h2>
          <p className="login-subtitle">
            {isRegister ? "创建账号开始对战" : "登录以开始对战"}
          </p>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label>用户名</label>
            <div className="input-group">
              <UserIcon className="input-icon" />
              <input
                type="text"
                value={username}
                onChange={(e) => {
                  setUsername(e.target.value);
                  clearError("username");
                }}
                placeholder="输入你的用户名"
                disabled={loading}
                autoFocus
                autoComplete="username"
              />
            </div>
            {errors.username && (
              <div className="field-error" role="alert">
                <ExclamationCircleIcon className="error-icon" />
                <span>{errors.username}</span>
              </div>
            )}
          </div>

          <div className="form-group">
            <label>密码</label>
            <div className="input-group">
              <LockIcon className="input-icon" />
              <input
                type={showPassword ? "text" : "password"}
                value={password}
                onChange={(e) => {
                  setPassword(e.target.value);
                  clearError("password");
                }}
                placeholder={isRegister ? "至少 6 个字符" : "输入你的密码"}
                disabled={loading}
                autoComplete={isRegister ? "new-password" : "current-password"}
              />
              <button
                type="button"
                className="toggle-password"
                onClick={() => setShowPassword(!showPassword)}
                aria-label={showPassword ? "隐藏密码" : "显示密码"}
              >
                {showPassword ? <EyeOffIcon className="toggle-icon" /> : <EyeIcon className="toggle-icon" />}
              </button>
            </div>
            {errors.password && (
              <div className="field-error" role="alert">
                <ExclamationCircleIcon className="error-icon" />
                <span>{errors.password}</span>
              </div>
            )}
          </div>

          <div className={`form-group confirm-password-field ${isRegister ? "" : "hidden"}`}>
            <label>确认密码</label>
            <div className="input-group">
              <LockIcon className="input-icon" />
              <input
                type={showConfirmPassword ? "text" : "password"}
                value={confirmPassword}
                onChange={(e) => {
                  setConfirmPassword(e.target.value);
                  clearError("confirmPassword");
                }}
                placeholder="再次输入密码"
                disabled={loading}
                autoComplete="new-password"
              />
              <button
                type="button"
                className="toggle-password"
                onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                aria-label={showConfirmPassword ? "隐藏密码" : "显示密码"}
              >
                {showConfirmPassword ? <EyeOffIcon className="toggle-icon" /> : <EyeIcon className="toggle-icon" />}
              </button>
            </div>
            {errors.confirmPassword && (
              <div className="field-error" role="alert">
                <ExclamationCircleIcon className="error-icon" />
                <span>{errors.confirmPassword}</span>
              </div>
            )}
          </div>

          <div className="setup-buttons">
            <button type="submit" className="btn-primary" disabled={loading}>
              {loading
                ? "处理中..."
                : isRegister
                  ? "注册"
                  : "登录"}
            </button>
            <button type="button" className="btn-secondary" onClick={onBack} disabled={loading}>
              返回
            </button>
          </div>

          <button
            type="button"
            className="toggle-mode-action"
            onClick={toggleMode}
            disabled={loading}
          >
            {isRegister ? "去登录" : "立即注册"}
          </button>
        </form>
      </div>
    </div>
  );
}

export default LoginScreen;
