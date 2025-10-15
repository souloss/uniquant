
[TOC]

# 🦀 Uniquant 项目开发指南
> 本文档旨在帮助开发者快速了解项目架构、开发规范及常见工作流，保证团队成员能够以一致的方式进行高效协作与维护。

## 1. 概述

### 1.1 项目简介
Uniquant 是一个基于 **Axum + SeaORM + Tower + Tokio** 的高性能异步后端服务；
项目遵循 DDD 分层架构设计(尽可能)，包括：

- `api`：接口层（HTTP 路由与控制器）
  - 负责 HTTP 交互、认证、中间件、序列化。
  - 仅负责输入/输出（DTO、HTTP 绑定）；
  - 业务逻辑由 Service 层完成；
- `service`：业务逻辑层
  - 负责协调多个领域模型与仓储（Repository）执行完整的业务操作。
  - 不直接依赖数据库等外部资源，而是依赖仓储/事务接口
- `dto`：数据传输与领域对象定义
  - 定义核心业务对象、DTOs（数据传输对象）、聚合根等。
- `db`：数据库访问层（Repository）
  - 基础设施层，负责外部资源访问(DB部分)
- `error`：统一错误定义
- `i18n`：多语言支持

### 1.2 技术栈

| 模块 | 技术栈 |
|------|--------|
| Web 框架 | [Axum](https://github.com/tokio-rs/axum) |
| ORM | [SeaORM](https://www.sea-ql.org/SeaORM/) |
| 异步运行时 | [Tokio](https://tokio.rs) |
| 日志与追踪 | [tracing](https://docs.rs/tracing) |
| 中间件 | [tower-http](https://docs.rs/tower-http) |
| 多语言 | [fluent-bundle](https://projectfluent.org/) |
| 构建与运行 | Cargo + Makefile |


## 2. 数据库管理
### sea-orm-cli 的使用
安装 sea-orm-cli 后，通过 sea-orm-cli 定义 migration，进行数据库表结构的升级/降级管理：
- 通过 sea-orm-cli migrate init [-d dir] 在指定目录中生成一个新的迁移 crate；
- 执行 sea-orm-cli migrate generate [-d dir] <MIGRATION_NAME> 生成一个新的迁移文件；该文件的命名规则为 `m<datetime>_<migration_name>.rs`；；
- 修改上面生成的迁移文件，在里面编写数据库表结构的变更代码；实现 `up` 实现表升级逻辑，实现 `down` 实现表降级逻辑；
  - 无论是创建，还是字段修改，索引创建都可，但需要尽可能保证 up 和 down 互为逆逻辑；
  - 开发阶段，可以直接在同一个迁移文件中编写多个表的升级/降级逻辑，删除本地数据库重新 up 使之应用最新的逻辑即可；
  - 生产阶段，每次变更都需要写在新的 migration 中，以保证生产环境的平滑升级；
  - 通过 sea-orm-cli migrate up/down/refresh 可以升级，降级，重置表结构；通过 sea-orm-cli migrate status 查看表结构版本状态(哪些migration已应用，哪些未应用)；
- 执行 sea-orm-cli generate entity [-o dir] [--lib] 在指定目录生成数据库实体模型，加上 --lib 表示生成一个 crate，否则会生成一个 mod；通过不同选项可以生成不同的实体：
  - --compact-format 表示生成紧凑格式的实体代码
  - --expanded-format 表示生成展开格式的实体代码，一般用于研究实体封装和运行原理时使用
  - --frontend-format 表示生成前端格式的实体代码，也就是纯结构体
  - 还有一些选项可以选择或忽略需要生成实体的表，可以自定义序列化/反序列化逻辑，时间格式，需要额外加上的派生宏或属性等

> 为了方便项目使用，我将常见使用方法，生成目录位置，数据库URL的定义放到了 Makefile 中；

> 注：因为这种数据库表升级方式不会备份数据，删除列等操作也无法回退，所以这并不是大型互联网项目的最佳实践，但是对于小型项目，或者快速原型开发来说，这是一个非常方便的做法。


### db 层的封装和使用
目前 db 层封装代码在 [src/db](../src/db) 目录下：
- connection.rs: 具体的数据库连接池的定义和初始化逻辑；
 - 通过数据库配置文件中的配置项，生成一个连接池实体，用于：
   - 为其它模块提供数据库连接；
   - 提供 run_migrations 方法，用于 main 中自动进行数据库迁移，保证每次程序运行时，数据库表结构都是最新的；

- TODO
  - 事务封装（Transaction API）

## 3. Repository 层
### repo 层的封装和使用
在 迁移(migration) 代码中定义好表结构，并且应用到数据库，以及根据数据库生成实体代码后，就可以着手 repo 层的封装了；

目前 repo 层封装代码在 [src/db/repositories](../src/db/repositories) 目录下：
- mod.rs：repositories 命名空间，定义了 Repository 基类，通过泛型 Trait 的形式，封装通用的增删改查方法；
- xxxx.rs: 具体的实体 repo 层定义，目前有个 InstrumentRepository 作为示例代码；

Repository 实现示例：
```rust
pub struct InstrumentRepository {
    db: Arc<DbPool>,
}

impl InstrumentRepository {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self { db }
    }
    
    // 自定义查询方法
    pub async fn find_by_exchange_and_symbol(&self, exchange: &str, symbol: &str) -> Result<Option<Model>, DbErr> {
        // 实现特定查询逻辑
    }
}

// 实现通用 Repository trait
impl Repository<Entity, Model, ActiveModel> for InstrumentRepository {
    fn get_connection(&self) -> &DatabaseConnection {
        &self.db.conn
    }
    
    // 可以覆盖默认实现以提供自定义逻辑
}
```


## 4. API 层
API 层负责处理 HTTP 请求和响应，包括路由定义、请求验证、响应格式化等。

目前 API 层代码在 [src/api](../src/api) 目录下：
- `mod.rs`：定义了 WebServer 结构，负责初始化和启动 HTTP 服务器
- `middleware/`：中间件实现，如请求上下文、错误处理、认证等
- `instrument/`：交易标的相关的 API 路由和处理函数

API 路由示例：
```rust
// 在 WebServer::new 方法中注册路由
let app = Router::new()
    .route("/api/instruments", get(self::get_all).post(self::create))
    .route("/api/instruments/:id", get(self::get_by_id).put(self::update).delete(self::delete))
    // 添加中间件
    .layer(middleware::context::context_layer())
    .layer(middleware::error_handler::error_handler_layer());
```

API 处理函数示例：
```rust
// 处理创建请求
pub async fn create(
    State(service_factory): State<Arc<ServiceFactory>>,
    Json(payload): Json<CreateInstrumentRequest>,
) -> Result<Json<APIResponse<InstrumentResponse>>, AppError> {
    // 1. 获取服务实例
    let service = service_factory.instrument_service();
    // 2. 调用服务方法
    let result = service.create(payload).await?;
    // 3. 包装响应
    Ok(Json(APIResponse::success(result)))
}
```

## 5. DTO 层
DTO (Data Transfer Object) 层定义了用于数据传输的对象结构，包括请求对象、响应对象等。

目前 DTO 层代码在 [src/dto](../src/dto) 目录下：
- `mod.rs`：导出各个模块
- `response.rs`：定义通用响应结构 APIResponse
- `instrument.rs`：交易标的相关的 DTO 定义
- `generated/`：自动生成的 DTO 代码

DTO 示例：
```rust
// 请求对象
#[derive(Debug, Deserialize, Validate)]
pub struct CreateInstrumentRequest {
    #[validate(length(min = 1, max = 50))]
    pub exchange: String,
    #[validate(length(min = 1, max = 50))]
    pub symbol: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

// 响应对象
#[derive(Debug, Serialize)]
pub struct InstrumentResponse {
    pub id: i32,
    pub exchange: String,
    pub symbol: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 从数据库模型到响应对象的转换
impl From<Model> for InstrumentResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            exchange: model.exchange,
            symbol: model.symbol,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
```

## 6. Service 层

### service 层的封装和使用
Service 层负责封装业务逻辑，调用 Repository 实现具体业务操作。

目前 service 层代码在 [src/service](../src/service) 目录下：
- `mod.rs`：定义了 `APPResult<T>` 类型，统一 Service 层的返回类型
- `factory.rs`：服务工厂，负责创建和管理各种服务实例，实现依赖注入
- `instrument.rs`：交易标的服务，提供创建、查询、更新等功能

服务层实现示例：
```rust
// 服务结构定义
pub struct InstrumentService {
    repo: Arc<InstrumentRepository>,
}

impl InstrumentService {
    // 构造函数，接收仓储实例
    pub fn new(repo: Arc<InstrumentRepository>) -> Self {
        Self { repo }
    }
    
    // 业务方法实现
    pub async fn create(&self, new_instrument: CreateInstrumentRequest) -> APPResult<InstrumentResponse> {
        // 1. 业务规则验证
        // 2. 调用仓储层
        // 3. 转换并返回结果
    }
}
```

服务工厂使用示例：
```rust
// 在应用启动时初始化
let service_factory = service::factory::ServiceFactory::new(repo.clone());
let service_factory = Arc::new(service_factory);

// 在需要使用服务时
let instrument_service = service_factory.instrument_service();
```

## 7. 错误处理
目前的错误处理的唯一真实源定义在 `configs/codes.yaml` 中，通过它定义 **应用级的错误码(枚举变体形式)** 以及 **描述**、**翻译** 等信息，方便代码生成器([tools/ftl-codegen](./tools/ftl-codegen))生成统一的代码；
错误码的定义基于 [thiserror](https://docs.rs/thiserror/latest/thiserror/) 过程宏库，它能为枚举错误生成 `Error trait` 和 `Display` 等样板代码的实现;

目前，修改 `configs/codes.yaml` 文件后，通过 `make generate-appcode` 命令即可生成错误码相关代码，以及翻译文件；

生成的错误码仅用于处理领域错误，也就是主要用于 Service 层；我在 `service/mod.rs` 中定义了 `type APPResult<T> = Result<T, AppError>;` 用于统一 Service 的返回类型；

> 后续需要支持错误回溯以及底层错误类型封装可以考虑使用 [snafu](https://docs.rs/snafu/latest/snafu/index.html) 进行重构；


## 8. 多语言支持
多语言支持是一项单独的特性，不与任何模块耦合；为了约束多语言的能力与使用，我定义了 `pub trait I18nBackend: Send + Sync` 表示翻译后端；它支持的特性有：
- 设置和获取当前语言，也就是默认的翻译语言；
- 通过 Key 和可选的语言(locale)和参数(args)获取对应语言的翻译文本；
- 通过 GlobalI18n 管理翻译全局实例，main 中进行初始化后整个程序都能使用；

多语言实现位于 [src/i18n](../src/i18n) 目录：
- `mod.rs`：定义 I18nBackend trait 和 GlobalI18n 单例
- `fluent.rs`：基于 Fluent 的多语言实现

使用示例：
```rust
// 初始化（在 main.rs 中）
let locales_path = Path::new(config.configs_dir.as_str()).join("locales");
let i18n_backend = FluentBackend::new(locales_path.as_os_str().to_str().unwrap(), "en").unwrap();
GlobalI18n::get().init_with_backend(i18n_backend)?;

// 在代码中使用
let message = t("error-not_found", None).unwrap_or("Resource not found".to_string());
// 或者指定语言
let message = t_locale("error-not_found", &langid!("zh-CN"), None).unwrap_or("资源未找到".to_string());
```

## 9. 项目开发流程

### 9.1 新功能开发流程

1. **数据库设计**
   - 通过 `make migrate-add MIG_NAME=<migration_name>` 在 `crates/migration/src/m*_*.rs` 中生成新的迁移文件
   - 实现 `up` 和 `down` 方法定义表结构变更
   - 运行 `make migrate-up` 将表变更应用到本地开发数据库

2. **实体生成**
   - 运行 `make generate-entity` 生成实体代码
   - 运行 `make generate-dto` 生成DTO代码
   - 实体代码将生成到 `crates/entities/src` 目录

3. **Repository 层开发**
   - 在 `src/db/repositories` 中创建新的仓储实现
   - 实现 Repository trait 和自定义查询方法

4. **DTO 层开发**
   - 在 `src/dto` 中定义请求和响应对象
   - 实现与实体模型的转换方法

5. **Service 层开发**
   - 在 `src/service` 中实现业务逻辑
   - 在 `service/factory.rs` 中添加服务实例创建方法

6. **API 层开发**
   - 在 `src/api` 中添加路由和处理函数
   - 在 `api/mod.rs` 中注册新路由

7. **错误处理**
   - 如需新的错误类型，在 `configs/codes.yaml` 中添加
   - 运行 `make generate-appcode` 生成错误代码