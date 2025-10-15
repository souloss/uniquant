# --- SUCCESS ---
## Code: 0
success-success = 操作成功

# --- ERROR ---
## Code: 4000
error-bad_request = 请求错误: { $message }

## Code: 4001
error-unauthorized = 未授权，您没有执行该操作的权限。

## Code: 4003
error-forbidden = 访问该资源被禁止。

## Code: 4004
error-not_found = { $identifier ->
    [none] 未找到资源 { $resource }。
   *[other] 未找到资源 { $resource }（标识符：{ $identifier }）。
}


## Code: 4009
error-conflict = 资源 { $resource }（标识符：{ $identifier }）已存在。

## Code: 4010
error-validation = 校验失败。

## Code: 5000
error-internal = 服务器内部错误，请稍后再试。

## Code: 5001
error-database = 数据库错误: { $message }

