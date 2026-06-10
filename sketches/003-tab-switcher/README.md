## Variant: 分类Tab切换

### Design stance
顶部Tab切换四个类别，每个类别用表格展示完整列表，信息密度最高。

### Key choices
- Layout: 单栏，顶部统计 + Tab切换 + 表格列表
- Typography: 系统字体，表格紧凑排版
- Color: 浅灰背景 + 白色表格，状态色：红/橙/绿
- Interaction: Tab切换类别，快速筛选标签，点击行展开详情

### Trade-offs
- Strong at: 信息密度最高，适合数据量大的场景
- Weak at: 首页不够直观，需要切换Tab查看各类别

### Best for
- 需要查看详细列表的用户
- 数据量大、需要筛选和排序的场景

### 首页展示数量建议
每类显示全部记录，支持筛选（已过期/即将到期/正常）。
