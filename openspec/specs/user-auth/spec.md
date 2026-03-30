# Capability: User Authentication

## ADDED Requirements

### Requirement: User registration with username and password
The system SHALL allow new users to register an account using a username and password.

#### Scenario: Successful registration
- **WHEN** a user provides a valid username (3-20 characters) and password (minimum 6 characters)
- **THEN** the system creates a new user account with a unique user ID
- **AND** the password is stored as a bcrypt hash
- **AND** the system returns a success response

#### Scenario: Registration with existing username
- **WHEN** a user attempts to register with a username that already exists
- **THEN** the system rejects the registration
- **AND** returns an error message "用户名已存在"

#### Scenario: Registration with invalid username
- **WHEN** a user provides a username with fewer than 3 characters or more than 20 characters
- **THEN** the system rejects the registration
- **AND** returns an error message describing valid username length

#### Scenario: Registration with weak password
- **WHEN** a user provides a password with fewer than 6 characters
- **THEN** the system rejects the registration
- **AND** returns an error message "密码至少需要 6 个字符"

### Requirement: User login with username and password
The system SHALL allow registered users to authenticate using their username and password.

#### Scenario: Successful login
- **WHEN** a user provides correct username and password
- **THEN** the system generates a session token (UUID)
- **AND** stores the token in memory associated with the user ID
- **AND** returns the token, user ID, and username to the client

#### Scenario: Login with incorrect password
- **WHEN** a user provides an incorrect password
- **THEN** the system rejects the login attempt
- **AND** returns an error message "用户名或密码错误"
- **AND** does not reveal whether the username exists

#### Scenario: Login with non-existent username
- **WHEN** a user provides a username that does not exist
- **THEN** the system rejects the login attempt
- **AND** returns an error message "用户名或密码错误"
- **AND** does not reveal whether the username exists

### Requirement: Session token validation
The system SHALL validate session tokens for protected API endpoints.

#### Scenario: Valid token access
- **WHEN** a client includes a valid session token in the Authorization header
- **THEN** the system allows access to the protected resource
- **AND** associates the request with the authenticated user ID

#### Scenario: Invalid token access
- **WHEN** a client includes an invalid or expired session token
- **THEN** the system rejects the request
- **AND** returns HTTP 401 Unauthorized

#### Scenario: Missing token access
- **WHEN** a client requests a protected resource without an Authorization header
- **THEN** the system rejects the request
- **AND** returns HTTP 401 Unauthorized

### Requirement: Session storage in memory
The system SHALL store active session tokens in server memory.

#### Scenario: Session lookup
- **WHEN** the system needs to validate a token
- **THEN** it retrieves the associated user ID from the in-memory session store

#### Scenario: Server restart clears sessions
- **WHEN** the server restarts
- **THEN** all in-memory session data is cleared
- **AND** users must log in again

### Requirement: Password security with bcrypt
The system SHALL store passwords using bcrypt hash algorithm.

#### Scenario: Password hashing on registration
- **WHEN** a user registers with a password
- **THEN** the password is hashed using bcrypt with appropriate cost factor
- **AND** only the hash is stored in the database

#### Scenario: Password verification on login
- **WHEN** a user attempts to log in
- **THEN** the system verifies the provided password against the stored bcrypt hash
