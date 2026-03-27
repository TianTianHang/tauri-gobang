import "./MainMenu.css";
import { RobotIcon, HomeIcon, GlobeIcon } from "./Icons";

interface MainMenuProps {
  onPlayAI: () => void;
  onHostGame: () => void;
  onJoinGame: () => void;
}

function MainMenu({ onPlayAI, onHostGame, onJoinGame }: MainMenuProps) {
  return (
    <div className="main-menu">
      <div className="menu-container">
        <h1 className="menu-title">五子棋</h1>
        <p className="menu-subtitle">Gobang / Five in a Row</p>

        <div className="menu-buttons">
          <button className="menu-btn" onClick={onPlayAI}>
            <RobotIcon className="btn-icon" />
            <span className="btn-text">人机对战</span>
            <span className="btn-desc">挑战 AI 棋手</span>
          </button>
          <button className="menu-btn" onClick={onHostGame}>
            <HomeIcon className="btn-icon" />
            <span className="btn-text">创建房间</span>
            <span className="btn-desc">作为房主等待对手</span>
          </button>
          <button className="menu-btn" onClick={onJoinGame}>
            <GlobeIcon className="btn-icon" />
            <span className="btn-text">加入房间</span>
            <span className="btn-desc">连接到对手的房间</span>
          </button>
        </div>
      </div>
    </div>
  );
}

export default MainMenu;
