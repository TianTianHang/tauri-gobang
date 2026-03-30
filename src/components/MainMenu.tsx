import "./MainMenu.css";
import { RobotIcon, GlobeIcon } from "./Icons";

interface MainMenuProps {
  onPlayAI: () => void;
  onOnlinePlay: () => void;
}

function MainMenu({ onPlayAI, onOnlinePlay }: MainMenuProps) {
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
          <button className="menu-btn" onClick={onOnlinePlay}>
            <GlobeIcon className="btn-icon" />
            <span className="btn-text">联机对战</span>
            <span className="btn-desc">通过服务器匹配对手</span>
          </button>
        </div>
      </div>
    </div>
  );
}

export default MainMenu;
