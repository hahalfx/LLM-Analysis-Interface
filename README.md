# LLM 分析界面

基于 Next.js 构建的语音交互大模型测试样本管理与分析平台，专注于评估车载语音助手的响应质量。

## 功能特性

- **测试样本管理**：创建、编辑、删除和导入测试语料
- **音频播放与录制**：支持播放测试指令音频并录制车机响应
- **语音识别集成**：集成讯飞语音识别API，实时转写车机响应
- **自动化测试流程**：支持一键启动自动化测试流程
- **多维度评分系统**：基于语义正确性、状态变更确认和表达无歧义性进行评分
- **实时分析结果可视化**：直观展示评估结果和改进建议
- **批量测试支持**：可选择多个测试样本进行批量测试
- **响应式布局**：适配不同设备屏幕尺寸
- **暗黑/明亮主题**：支持主题切换，提升用户体验
- **交互式数据表格**：支持排序、过滤和分页功能

## 技术栈

- **框架**: Next.js 15.2.4 (App Router)
- **UI 组件**: Radix UI 原子组件 + Shadcn UI 组件系统
- **样式**: Tailwind CSS 3.4 + tailwindcss-animate 动画插件
- **状态管理**: React Hooks + Context API
- **表单处理**: React Hook Form + Zod 验证
- **数据表格**: TanStack Table v8 (原React Table)
- **可视化**: Recharts 2.15 图表库
- **工具库**: 
  - date-fns: 日期处理
  - clsx/tailwind-merge: 条件类名管理
  - XLSX: Excel文件导入导出
- **类型系统**: TypeScript 5 严格类型检查

## 系统架构图

```mermaid
flowchart TD
    subgraph "前端 (Next.js)"
        UI[用户界面组件]
        Hooks[React Hooks]
        Services[API服务层]
    end
    
    subgraph "API路由 (Next.js API Routes)"
        TestSamples[测试样本API]
        AudioFiles[音频文件API]
        Analysis[分析API]
        XunfeiASR[讯飞语音识别API]
    end
    
    subgraph "外部服务"
        XunfeiCloud[讯飞云服务]
        LLMService[大模型分析服务]
    end
    
    UI --> Hooks
    Hooks --> Services
    Services --> TestSamples
    Services --> AudioFiles
    Services --> Analysis
    Services --> XunfeiASR
    XunfeiASR --> XunfeiCloud
    Analysis --> LLMService
```

## 测试流程图

```mermaid
sequenceDiagram
    participant User as 用户
    participant UI as 界面组件
    participant Audio as 音频系统
    participant ASR as 语音识别
    participant API as 分析API
    participant LLM as 大模型

    User->>UI: 选择测试样本
    User->>UI: 点击"开始自动化测试"
    UI->>Audio: 播放测试指令音频
    Audio-->>User: 播放完成
    UI->>ASR: 自动开始录音
    User-->>ASR: 车机响应语音
    ASR->>UI: 实时转写文本
    ASR->>UI: 检测到稳定结果
    UI->>API: 提交分析请求
    API->>LLM: 请求大模型分析
    LLM-->>API: 返回分析结果
    API-->>UI: 返回评分和建议
    UI-->>User: 显示分析结果
    UI->>Audio: 自动播放下一个样本
```

## 组件关系图

```mermaid
flowchart TD
    App[App] --> LLMInterface[LLMAnalysisInterface]
    
    LLMInterface --> NavTabs[NavTabs]
    LLMInterface --> TestSamples[TestSamples]
    LLMInterface --> MachineResponse[MachineResponse]
    LLMInterface --> ProgressBar[ProgressBar]
    LLMInterface --> AnalysisResults[AnalysisResults]
    
    TestSamples --> DataTable[DataTable]
    AnalysisResults --> ScoreDisplay[ScoreDisplay]
    
    MachineResponse -.-> |ref| LLMInterface
    ProgressBar -.-> |props| LLMInterface
```

## 技术实现细节

### 前端架构

- **组件设计模式**：采用组合式组件设计，将UI与业务逻辑分离
- **状态管理策略**：使用React Hooks管理本地状态，避免复杂状态管理库
- **响应式设计**：基于Tailwind的响应式类和Flex/Grid布局实现
- **主题系统**：使用CSS变量和next-themes实现深色/浅色主题切换
- **性能优化**：
  - 组件懒加载
  - 使用useRef避免不必要的重渲染
  - 使用React.memo优化列表渲染

### 语音处理流程

1. **音频播放**：使用Web Audio API播放测试指令音频文件
2. **语音录制**：使用MediaRecorder API录制车机响应
3. **实时转写**：
   - 通过WebSocket连接讯飞语音识别服务
   - 将音频数据实时发送至服务端
   - 处理动态修正的识别结果
4. **稳定性检测**：实现识别结果稳定性检测算法，自动停止录音并提交分析

### API集成架构

- **RESTful API设计**：遵循REST原则设计API端点
- **错误处理机制**：统一的错误捕获和友好提示
- **模拟数据回退**：当API不可用时提供模拟数据保证功能可用性
- **API路由实现**：使用Next.js API Routes实现后端服务

### 测试评估系统

- **多维度评分模型**：
  - 语义正确性：评估响应内容与用户指令的语义匹配程度
  - 状态变更确认：评估系统是否明确确认了状态变更
  - 表达无歧义性：评估响应表达是否清晰无歧义
- **综合评分算法**：基于加权平均计算总体评分
- **改进建议生成**：基于评分结果自动生成针对性改进建议

## 快速开始

### 环境要求
- Node.js 18+
- npm 9+

### 环境变量配置
在项目根目录下创建.env.local文件，并添加以下内容：
```
XUN_FEI_APPID='你的讯飞AppID'
XUN_FEI_API_SECRET='你的讯飞API密钥'
XUN_FEI_API_KEY='你的讯飞API密钥'
```

### 安装步骤
```bash
# 克隆仓库
git clone [仓库地址]

# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

访问 http://localhost:3000 查看应用

## 项目结构
```
/LLM Analysis Interface
├── app/                    # Next.js 应用路由
│   ├── api/                # API路由端点
│   │   ├── analyze/        # 分析API
│   │   ├── audio-files/    # 音频文件API
│   │   ├── test-samples/   # 测试样本API
│   │   └── xunfei-asr/     # 讯飞语音识别API
│   ├── globals.css         # 全局样式
│   ├── layout.tsx          # 应用布局
│   └── page.tsx            # 主页面
├── components/             # 可复用组件
│   ├── ui/                 # Shadcn UI 基础组件
│   ├── analysis-results.tsx # 分析结果展示组件
│   ├── llm-analysis-interface.tsx # 主界面组件
│   ├── machine-response.tsx # 车机响应组件
│   ├── progress-bar.tsx    # 进度条组件
│   ├── score-display.tsx   # 评分展示组件
│   ├── test-samples.tsx    # 测试样本管理组件
│   └── theme-toggle.tsx    # 主题切换组件
├── services/               # API 服务层
│   └── api.ts              # API调用封装
├── types/                  # TypeScript 类型定义
│   └── api.ts              # API相关类型定义
├── public/                 # 静态资源
│   └── audio/              # 测试音频文件
└── styles/                 # 全局样式
```

## 核心功能实现

### 语音识别流程
1. 用户点击麦克风按钮开始录音
2. 通过WebSocket连接讯飞语音识别服务
3. 实时处理音频数据并发送至服务端
4. 接收并处理识别结果，支持动态修正
5. 检测识别结果稳定性，自动停止录音并提交分析

### 自动化测试流程
1. 用户选择测试样本并点击"开始自动化测试"
2. 系统自动播放当前测试样本对应的音频文件
3. 音频播放结束后自动开始录音，捕获车机响应
4. 语音识别结果稳定后自动提交分析
5. 分析完成后自动进入下一个测试样本

### 评分系统实现
- 使用大模型分析车机响应质量
- 基于预设评分标准计算多维度评分
- 生成针对性改进建议
- 可视化展示评分结果和详细分析

## 开发脚本
```bash
# 启动开发服务器
npm run dev

# 生产构建
npm run build

# 启动生产服务器 
npm run start

# 代码检查
npm run lint
```

## 部署说明
本应用支持多种部署方式：
- Vercel平台一键部署
- Docker容器化部署
- 传统Node.js服务器部署

## 贡献指南
欢迎提交Pull Request，请确保：
1. 遵循现有代码风格和组件设计模式
2. 添加完整的TypeScript类型定义
3. 更新相关文档
4. 提供必要的测试用例

## 项目进展
1. 将Task交由Redux管理(✅)
2. 新建测试任务管理界面(✅)
3. 将Task状态运用到LLManalysis中(✅)(currenttask与itmes同步问题)
4. 完善新建任务界面(✅)
5. 将测试任务管理界面的最近验证任务改为认证任务的data-table(✅)
6. 完善唤醒词功能(✅)