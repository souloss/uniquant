# --- SUCCESS ---
## Code: 0
success-success = Success

# --- ERROR ---
## Code: 4000
error-bad_request = Bad request: { $message }

## Code: 4001
error-unauthorized = You are not authorized to perform this action.

## Code: 4003
error-forbidden = Access to this resource is forbidden.

## Code: 4004
error-not_found = { $identifier ->
    [none] The { $resource } was not found.
   *[other] The { $resource } with identifier "{ $identifier }" was not found.
}


## Code: 4009
error-conflict = The { $resource } with identifier "{ $identifier }" already exists.

## Code: 4010
error-validation = Validation failed.

## Code: 5000
error-internal = An internal server error occurred. Please try again later.

## Code: 5001
error-database = A database error occurred: { $message }

