# TUI 重新设计方案

## 设计目标

根据使用频次重新设计TUI展示，将不常用的资源类型归类到"更多资源"面板中，使界面更加清晰和易于使用。

## 设计原则

1. 核心资源类型（高频使用）在主标签页中直接显示
2. 不常用资源类型（低频使用）归类到"更多资源"面板
3. 通过数字键快速访问所有资源类型
4. 保持界面简洁，减少视觉干扰

## 资源类型分类

### 核心资源类型（主标签页直接显示）
- Namespaces（命名空间）
- Pods（容器组）
- Services（服务）
- Deployments（部署）

### 更多资源类型（在"更多资源"面板中显示）
1. PVCs - 持久化存储声明
2. PVs - 持久化存储卷
3. Nodes - 节点管理
4. ConfigMaps - 配置管理
5. Secrets - 密钥管理
6. Jobs - 任务管理
7. DaemonSets - 守护进程集

## 交互设计

### 标签页导航
- 主标签页：Namespaces → Pods → Services → Deployments → More Resources → Help
- 使用Tab/Shift+Tab在主标签页间循环切换
- 使用F6快速访问"更多资源"面板
- 使用F1-F5快速访问特定资源类型（Nodes, ConfigMaps, Secrets, Jobs, DaemonSets）

### 更多资源面板操作
- 使用数字键1-7快速访问对应资源类型
- 使用j/k或方向键在资源列表中导航
- 使用Enter键访问当前选中的资源类型
- 使用Esc键返回主面板

### 底部说明栏
根据不同模式显示相应的操作提示：
- 更多资源面板：显示"1-7 访问资源 • Tab/Shift+Tab 切换面板 • Esc 返回 • q 退出"

## 技术实现

### 新增组件
- [more_resources.rs](file:///Users/zhanglianwei/github/kube/src/ui/components/more_resources.rs)：实现更多资源面板的UI组件

### 修改的文件
- [app.rs](file:///Users/zhanglianwei/github/kube/src/app.rs)：
  - 添加[selected_more_resource_index](file:///Users/zhanglianwei/github/kube/src/app.rs#L111-L111)字段
  - 添加处理更多资源面板导航的逻辑
  - 更新键盘事件处理逻辑
- [ui/mod.rs](file:///Users/zhanglianwei/github/kube/src/ui/mod.rs)：
  - 更新[render_header](file:///Users/zhanglianwei/github/kube/src/ui/mod.rs#L73-L131)函数，重新设计标签页显示逻辑
  - 更新[render_main_content](file:///Users/zhanglianwei/github/kube/src/ui/mod.rs#L133-L165)函数，添加更多资源面板渲染
  - 更新[render_footer](file:///Users/zhanglianwei/github/kube/src/ui/mod.rs#L167-L267)函数，添加更多资源面板的提示信息
- [ui/components/mod.rs](file:///Users/zhanglianwei/github/kube/src/ui/components/mod.rs)：声明more_resources模块

## 用户体验优化

1. 界面更加简洁，只显示最常用的资源类型
2. 通过数字键快速访问，提高操作效率
3. 清晰的提示信息，帮助用户理解和使用新功能
4. 保持与其他面板一致的交互方式