import { useState, useEffect } from 'react';
import './styles/global.css';
import { AddModal } from './components/AddModal';
import { invoke } from '@tauri-apps/api/core';

interface Game {
  id: string;
  name: string;
}

interface GameRecord {
  id: string;
  game_id: string;
  type: string;
  purchase_date?: string;
  start_date?: string;
  expire_date?: string;
  amount?: number;
  note?: string;
}

function App() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [games, setGames] = useState<Game[]>([]);
  const [gameRecords, setGameRecords] = useState<GameRecord[]>([]);
  const [message, setMessage] = useState('');

  // 初始化数据库
  useEffect(() => {
    const init = async () => {
      try {
        await invoke('init_db');
        await loadGames();
        await loadGameRecords();
      } catch (err) {
        console.error('初始化失败:', err);
      }
    };
    init();
  }, []);

  const loadGames = async () => {
    try {
      const result = await invoke('get_games');
      setGames(result as Game[]);
    } catch (err) {
      console.error('加载游戏失败:', err);
    }
  };

  const loadGameRecords = async () => {
    try {
      // 获取所有游戏的记录
      const allRecords: GameRecord[] = [];
      for (const game of games) {
        try {
          const records = await invoke('get_game_records', { gameId: game.id });
          allRecords.push(...(records as GameRecord[]));
        } catch (err) {
          // 忽略单个游戏的错误
        }
      }
      setGameRecords(allRecords);
    } catch (err) {
      console.error('加载记录失败:', err);
    }
  };

  const handleAdd = async (type: string, data: any) => {
    try {
      if (type === 'game') {
        // 1. 先创建或获取游戏
        let gameId = '';
        const existingGame = games.find(g => g.name === data.name);
        
        if (existingGame) {
          gameId = existingGame.id;
        } else {
          // 创建新游戏
          const newGame = await invoke('add_game', { name: data.name, icon: null });
          gameId = (newGame as Game).id;
          await loadGames();
        }

        // 2. 添加消费记录
        await invoke('add_game_record', {
          gameId,
          recordType: data.recordType,
          purchaseDate: data.purchaseDate || null,
          startDate: data.startDate || null,
          expireDate: data.expireDate || null,
          amount: data.amount ? parseFloat(data.amount) : null,
          note: data.note || null,
        });

        // 3. 刷新记录
        await loadGameRecords();
        
        setMessage('添加成功！');
        setTimeout(() => setMessage(''), 2000);
      } else {
        // 其他类型的添加逻辑
        setMessage('暂不支持该类型');
        setTimeout(() => setMessage(''), 2000);
      }
    } catch (err) {
      console.error('添加失败:', err);
      setMessage('添加失败: ' + String(err));
      setTimeout(() => setMessage(''), 3000);
    }
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
              <span className="nav-count">{gameRecords.length}</span>
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
          {/* 提示消息 */}
          {message && (
            <div className={`toast ${message.includes('成功') ? 'toast-success' : 'toast-error'}`}>
              {message}
            </div>
          )}

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
              <span className="stat-value">{gameRecords.length}</span>
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
            {gameRecords.length === 0 ? (
              <div className="empty-state">暂无记录，点击右上角「+ 添加」开始</div>
            ) : (
              <div className="card-list">
                {gameRecords.map((record) => {
                  const game = games.find(g => g.id === record.game_id);
                  return (
                    <div key={record.id} className="card">
                      <div className="card-top">
                        <span className="card-name">{game?.name || '未知游戏'} · {record.type}</span>
                        <span className="badge badge-success">正常</span>
                      </div>
                      <div className="card-meta">
                        {record.start_date} → {record.expire_date}
                      </div>
                      {record.amount && (
                        <div className="card-amount">消费 <strong>¥{record.amount}</strong></div>
                      )}
                    </div>
                  );
                })}
              </div>
            )}
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
