import { useState } from 'react';
import './styles/global.css';
import { AddModal } from './components/AddModal';

function App() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const handleAdd = (type: string, data: any) => {
    console.log('添加记录:', type, data);
    // TODO: 调用 API 添加记录
  };

  return (
    <div className="app">
      <aside className="sidebar">
        <div className="sidebar-header">
          <div className="sidebar-logo">⏱</div>
          <div className="sidebar-title">时停</div>
        </div>
        <nav className="sidebar-nav">
          <div className="nav-section">
            <div className="nav-label">概览</div>
            <div className="nav-item active">
              <span className="nav-icon">◎</span>
              <span className="nav-text">仪表盘</span>
            </div>
          </div>
          <div className="nav-section">
            <div className="nav-label">分类</div>
            <div className="nav-item">
              <span className="nav-icon">🎮</span>
              <span className="nav-text">游戏</span>
              <span className="nav-count">0</span>
            </div>
            <div className="nav-item">
              <span className="nav-icon">💻</span>
              <span className="nav-text">硬件质保</span>
              <span className="nav-count">0</span>
            </div>
            <div className="nav-item">
              <span className="nav-icon">🏠</span>
              <span className="nav-text">家用耗材</span>
              <span className="nav-count">0</span>
            </div>
            <div className="nav-item">
              <span className="nav-icon">💊</span>
              <span className="nav-text">药品</span>
              <span className="nav-count">0</span>
            </div>
          </div>
        </nav>
      </aside>

      <main className="main">
        <header className="toolbar">
          <div className="toolbar-left">
            <button className="btn btn-ghost">◀</button>
            <button className="btn btn-ghost">▶</button>
          </div>
          <div className="toolbar-center">仪表盘</div>
          <div className="toolbar-right">
            <input type="text" className="search-box" placeholder="搜索" />
            <button className="btn btn-primary" onClick={() => setIsModalOpen(true)}>+ 添加</button>
          </div>
        </header>

        <div className="content">
          <div className="stats-bar">
            <div className="stat">
              <span className="stat-value danger">0</span>
              <span className="stat-label">已过期</span>
            </div>
            <div className="stat">
              <span className="stat-value warning">0</span>
              <span className="stat-label">7天内</span>
            </div>
            <div className="stat">
              <span className="stat-value">0</span>
              <span className="stat-label">本月</span>
            </div>
            <div className="stat">
              <span className="stat-value">0</span>
              <span className="stat-label">总计</span>
            </div>
          </div>

          <div className="section">
            <div className="section-header">
              <div className="section-title">
                <span className="section-icon">🎮</span>
                游戏
              </div>
              <span className="section-link">显示全部 →</span>
            </div>
            <div className="empty-state">暂无记录，点击右上角「+ 添加」开始</div>
          </div>

          <div className="section">
            <div className="section-header">
              <div className="section-title">
                <span className="section-icon">💻</span>
                硬件质保
              </div>
              <span className="section-link">显示全部 →</span>
            </div>
            <div className="empty-state">暂无记录</div>
          </div>

          <div className="section">
            <div className="section-header">
              <div className="section-title">
                <span className="section-icon">🏠</span>
                家用耗材
              </div>
              <span className="section-link">显示全部 →</span>
            </div>
            <div className="empty-state">暂无记录</div>
          </div>

          <div className="section">
            <div className="section-header">
              <div className="section-title">
                <span className="section-icon">💊</span>
                药品
              </div>
              <span className="section-link">显示全部 →</span>
            </div>
            <div className="empty-state">暂无记录</div>
          </div>
        </div>
      </main>

      <AddModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onAdd={handleAdd}
      />
    </div>
  );
}

export default App;
