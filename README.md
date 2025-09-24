# UniQuant

**UniQuant** 是一个基于 **Rust + PostgreSQL** 构建的量化研究与交易平台，支持多种可交易标的物（股票、加密货币等），提供从数据存储到策略回测、模拟盘和实盘交易的一体化解决方案。

---

## ✨ 特性

- **多资产支持**：统一管理股票、加密货币及其他可交易资产  
- **高性能存储**：基于 PostgreSQL/TimescaleDB 管理行情、财报和情绪数据  
- **模块化分析**：可扩展的财务分析、健康诊断、安全性分析模块  
- **策略与信号**：支持多种量化策略、交易信号和指标计算  
- **回测系统**：历史行情回放与策略验证  
- **用户系统**：
  - 普通用户：使用自选股/币、策略配置、回测查询  
  - 超级用户：系统维护、权限管理、扩展功能  
- **未来功能**：
  - 财报解析与自然语言情绪分析
  - 模拟交易 / Paper Trading  
  - 实盘交易接入  
---

## 📂 模块结构（规划）
```
uniquant/
├── core/ # 核心库：策略框架、信号引擎
├── data/ # 数据管理：行情、财报、情绪
├── backtest/ # 回测引擎
├── execution/ # 模拟盘 & 实盘交易
├── api/ # 用户 API & FastAPI/Rust Axum 接口
├── web/ # Web 前端（可选）
└── docs/ # 文档
```


## 🗄️ 数据库设计（简要）
- `instrument`：可交易标的物（股票、货币、期货...）  
- `exchange`：交易所信息  
- `kline_xx`：多周期 K 线（1m, 1h, 1d ...）  
- `fundamental`：财报、基本面指标  
- `strategy_config`：用户自定义策略与参数  
- `backtest_result`：回测结果（JSON 指标存储）  
- `user`, `user_portfolio`：用户与自选资产  

### 环境依赖
- Rust (≥ 1.80)
- PostgreSQL (推荐 TimescaleDB 插件)
- Docker（可选）

### 架构图
```mermaid
flowchart TB
  subgraph DataCollectors["数据采集层"]
    A1[Exchange Collectors WS/REST]
  end

  subgraph Ingest["归一化 & 写入"]
    I1[Normalizer]
    I2[Writer TimescaleDB/Postgres]
  end

  subgraph Storage["存储层"]
    PG[(Postgres + TimescaleDB)]
    S3[(ObjectStorage S3 artifacts)]
    REDIS[(Redis short-lived cache)]
  end

  subgraph Services["核心服务"]
    API[Axum API Server]
    ANALYTICS[Analytics Modules Feature Store]
    SCHEDULER[Scheduler Cron Task Queue]
    WORKER[Backtest Worker Sandbox Executor]
  end

  subgraph Users["用户层"]
    Web[Web UI Dashboard]
    CLI[CLI SDK]
  end

  A1 --> I1 --> I2 --> PG
  ANALYTICS --> PG
  SCHEDULER --> WORKER
  API --> SCHEDULER
  API --> WORKER
  WORKER --> PG
  WORKER --> S3
  WORKER --> REDIS
  API --> REDIS
  Web --> API
  CLI --> API

  note left of REDIS
    短期缓存:
    - 信号序列
    - 结果缓存 (TTL 短，按需持久化到 S3)
  end
```
📈 路线图

- [ ] 基础数据采集（多交易所）
- [ ] K 线存储与查询
- [ ] 用户系统 & 自选股
- [ ] 策略定义与信号计算
- [ ] 回测系统
- [ ] 财报/情绪分析模块
- [ ] 模拟盘
- [ ] 实盘交易接入


## 🤝 贡献
欢迎提交 Issue 和 PR，一起完善 UniQuant 🚀