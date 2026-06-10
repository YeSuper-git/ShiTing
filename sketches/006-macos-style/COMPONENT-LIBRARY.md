# 时停 - 组件库

> 基于 macOS 26 原生应用风格

---

## 1. Sidebar 侧边栏

**用途**：应用主导航

**结构**：
```html
<aside class="sidebar">
  <div class="sidebar-header">
    <div class="sidebar-logo">图标</div>
    <div class="sidebar-title">时停</div>
  </div>
  <nav class="sidebar-nav">
    <div class="nav-section">
      <div class="nav-label">分组标签</div>
      <div class="nav-item active">
        <span class="nav-icon">图标</span>
        <span class="nav-text">名称</span>
        <span class="nav-count">数量</span>
      </div>
    </div>
  </nav>
</aside>
```

**样式规格**：
| 属性 | 值 |
|------|-----|
| 宽度 | 250px |
| 背景 | `rgba(30, 30, 30, 0.85)` + `blur(40px) saturate(180%)` |
| 文字颜色 | `rgba(255, 255, 255, 0.85)` |
| 激活项背景 | `#007AFF` |
| 项目高度 | 28px |
| 项目圆角 | 8px |

**状态**：
- 默认：`rgba(255, 255, 255, 0.85)` 文字
- 悬浮：`rgba(255, 255, 255, 0.1)` 背景
- 激活：`#007AFF` 背景，白色文字

---

## 2. Toolbar 工具栏

**用途**：页面顶部操作栏

**结构**：
```html
<header class="toolbar">
  <div class="toolbar-left">
    <button class="btn btn-ghost">◀</button>
    <button class="btn btn-ghost">▶</button>
  </div>
  <div class="toolbar-center">标题</div>
  <div class="toolbar-right">
    <input class="search-box" placeholder="搜索">
    <button class="btn btn-primary">+ 添加</button>
  </div>
</header>
```

**样式规格**：
| 属性 | 值 |
|------|-----|
| 高度 | 56px |
| 背景 | `rgba(242, 242, 247, 0.8)` + `blur(20px) saturate(180%)` |
| 底部边框 | `1px solid rgba(0, 0, 0, 0.1)` |

---

## 3. Button 按钮

### 3.1 主按钮 (btn-primary)

**样式规格**：
| 属性 | 值 |
|------|-----|
| 背景 | `#007AFF` |
| 文字 | `#ffffff` |
| 高度 | 36px |
| 内边距 | `0 16px` |
| 圆角 | 8px |
| 字号 | 14px |
| 字重 | 500 |

**状态**：
- 悬浮：`#0056CC`
- 按下：`#0052B3`

### 3.2 幽灵按钮 (btn-ghost)

**样式规格**：
| 属性 | 值 |
|------|-----|
| 背景 | 透明 |
| 文字 | `#007AFF` |
| 高度 | 36px |
| 圆角 | 8px |

---

## 4. Card 卡片

**用途**：展示单条记录

**结构**：
```html
<div class="card">
  <div class="card-top">
    <span class="card-name">名称</span>
    <span class="card-badge badge-warning">状态</span>
  </div>
  <div class="card-meta">元信息</div>
  <div class="card-bottom">
    <div>
      <div class="card-countdown">28天</div>
      <div class="card-date">剩余</div>
    </div>
  </div>
</div>
```

**样式规格**：
| 属性 | 值 |
|------|-----|
| 背景 | `rgba(255, 255, 255, 0.8)` |
| 圆角 | 16px |
| 内边距 | 16px |
| 边框 | `1px solid rgba(0, 0, 0, 0.06)` |
| 最小宽度 | 240px |

**状态**：
- 默认：半透明白色背景
- 悬浮：上移 2px + `rgba(255, 255, 255, 0.95)` 背景 + 加深阴影

---

## 5. Badge 状态角标

**用途**：显示到期状态

**结构**：
```html
<span class="card-badge badge-expired">已过期</span>
<span class="card-badge badge-warning">即将到期</span>
<span class="card-badge badge-normal">正常</span>
```

**样式规格**：
| 状态 | 背景 | 文字 |
|------|------|------|
| 已过期 | `rgba(255, 59, 48, 0.12)` | `#FF3B30` |
| 即将到期 | `rgba(255, 149, 0, 0.12)` | `#FF9500` |
| 正常 | `rgba(52, 199, 89, 0.12)` | `#34C759` |

**通用规格**：
| 属性 | 值 |
|------|-----|
| 字号 | 11px |
| 字重 | 600 |
| 内边距 | `3px 8px` |
| 圆角 | 6px |

---

## 6. Search Box 搜索框

**样式规格**：
| 属性 | 值 |
|------|-----|
| 高度 | 36px |
| 宽度 | 200px |
| 背景 | `rgba(255, 255, 255, 0.8)` |
| 边框 | `1px solid rgba(0, 0, 0, 0.1)` |
| 圆角 | 10px |
| 字号 | 14px |

---

## 7. Modal 弹窗

**用途**：添加记录类型选择

**结构**：
```html
<div class="modal-overlay">
  <div class="modal">
    <div class="modal-header">
      <span class="modal-title">标题</span>
      <button class="modal-close">✕</button>
    </div>
    <div class="modal-body">内容</div>
  </div>
</div>
```

**样式规格**：
| 属性 | 值 |
|------|-----|
| 遮罩背景 | `rgba(0, 0, 0, 0.4)` + `blur(8px)` |
| 弹窗背景 | `rgba(245, 245, 247, 0.9)` + `blur(60px) saturate(200%)` |
| 弹窗圆角 | 20px |
| 弹窗宽度 | 480px |
| 阴影 | `0 20px 60px rgba(0, 0, 0, 0.15)` |

---

## 8. Type Card 类型选择卡片

**用途**：弹窗内的类型选择项

**结构**：
```html
<div class="type-card" onclick="selectType('game')">
  <div class="type-icon">🎮</div>
  <div class="type-name">游戏</div>
  <div class="type-desc">月卡、通行证、会员</div>
</div>
```

**样式规格**：
| 属性 | 值 |
|------|-----|
| 背景 | `rgba(0, 0, 0, 0.03)` |
| 圆角 | 12px |
| 内边距 | `18px 16px` |

**状态**：
- 悬浮：`rgba(0, 122, 255, 0.06)` 背景 + `rgba(0, 122, 255, 0.2)` 边框
- 按下：缩放 0.98

---

## 9. Summary Card 统计卡片

**用途**：顶部统计数据展示

**结构**：
```html
<div class="summary-card">
  <div class="summary-label">已过期</div>
  <div class="summary-value danger">3</div>
</div>
```

**样式规格**：
| 属性 | 值 |
|------|-----|
| 背景 | `#ffffff` |
| 圆角 | 12px |
| 内边距 | 18px |
| 阴影 | `0 1px 3px rgba(0,0,0,0.04)` |

---

## 10. Section Header 分组标题

**结构**：
```html
<div class="section-header">
  <div class="section-title">
    <span class="section-icon">🎮</span>
    游戏
  </div>
  <span class="btn-link">显示全部 →</span>
</div>
```

**样式规格**：
| 属性 | 值 |
|------|-----|
| 标题字号 | 18px |
| 标题字重 | 700 |
| 链接字号 | 13px |
| 链接颜色 | `#007AFF` |
