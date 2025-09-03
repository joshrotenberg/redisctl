# Redis Cloud API Implementation Tracking

This document tracks the implementation status of the Redis Cloud OpenAPI specification.

## Summary
- **Total API Paths**: 77
- **Total Component Schemas**: 121
- **Implementation Status**: Starting fresh incremental implementation

## API Paths by Category

### Account & Users (7 paths)
- [ ] `GET /` - Get current account info
- [ ] `GET /users` - List account users  
- [ ] `POST /users` - Create new user
- [ ] `GET /users/{userId}` - Get user details
- [ ] `PUT /users/{userId}` - Update user
- [ ] `DELETE /users/{userId}` - Delete user
- [ ] `GET /payment-methods` - List payment methods

### ACL - Access Control Lists (7 paths)
- [ ] `GET /acl/redisRules` - List Redis ACL rules
- [ ] `POST /acl/redisRules` - Create Redis ACL rule
- [ ] `GET /acl/redisRules/{aclRedisRuleId}` - Get Redis ACL rule
- [ ] `PUT /acl/redisRules/{aclRedisRuleId}` - Update Redis ACL rule
- [ ] `DELETE /acl/redisRules/{aclRedisRuleId}` - Delete Redis ACL rule
- [ ] `GET /acl/roles` - List ACL roles
- [ ] `POST /acl/roles` - Create ACL role
- [ ] `GET /acl/roles/{aclRoleId}` - Get ACL role
- [ ] `PUT /acl/roles/{aclRoleId}` - Update ACL role
- [ ] `DELETE /acl/roles/{aclRoleId}` - Delete ACL role
- [ ] `GET /acl/users` - List ACL users
- [ ] `POST /acl/users` - Create ACL user
- [ ] `GET /acl/users/{aclUserId}` - Get ACL user
- [ ] `PUT /acl/users/{aclUserId}` - Update ACL user
- [ ] `DELETE /acl/users/{aclUserId}` - Delete ACL user

### Cloud Accounts (2 paths)
- [ ] `GET /cloud-accounts` - List cloud accounts
- [ ] `POST /cloud-accounts` - Create cloud account
- [ ] `GET /cloud-accounts/{cloudAccountId}` - Get cloud account
- [ ] `PUT /cloud-accounts/{cloudAccountId}` - Update cloud account
- [ ] `DELETE /cloud-accounts/{cloudAccountId}` - Delete cloud account

### Fixed/Essentials Plans (11 paths)
- [ ] `GET /fixed/plans` - List available fixed plans
- [ ] `GET /fixed/plans/{planId}` - Get fixed plan details
- [ ] `GET /fixed/plans/subscriptions/{subscriptionId}` - Get plans for subscription
- [ ] `GET /fixed/redis-versions` - List Redis versions for fixed
- [ ] `GET /fixed/subscriptions` - List fixed subscriptions
- [ ] `POST /fixed/subscriptions` - Create fixed subscription
- [ ] `GET /fixed/subscriptions/{subscriptionId}` - Get fixed subscription
- [ ] `PUT /fixed/subscriptions/{subscriptionId}` - Update fixed subscription
- [ ] `DELETE /fixed/subscriptions/{subscriptionId}` - Delete fixed subscription
- [ ] `GET /fixed/subscriptions/{subscriptionId}/databases` - List databases
- [ ] `POST /fixed/subscriptions/{subscriptionId}/databases` - Create database
- [ ] `GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}` - Get database
- [ ] `PUT /fixed/subscriptions/{subscriptionId}/databases/{databaseId}` - Update database
- [ ] `DELETE /fixed/subscriptions/{subscriptionId}/databases/{databaseId}` - Delete database
- [ ] `POST /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/backup` - Backup database
- [ ] `POST /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/import` - Import to database
- [ ] `GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/slow-log` - Get slow log
- [ ] `GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags` - List tags
- [ ] `PUT /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags` - Update tags
- [ ] `DELETE /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags/{tagKey}` - Delete tag

### Pro Subscriptions (24 paths)
- [ ] `GET /subscriptions` - List subscriptions
- [ ] `POST /subscriptions` - Create subscription
- [ ] `GET /subscriptions/{subscriptionId}` - Get subscription
- [ ] `PUT /subscriptions/{subscriptionId}` - Update subscription
- [ ] `DELETE /subscriptions/{subscriptionId}` - Delete subscription
- [ ] `GET /subscriptions/redis-versions` - List Redis versions
- [ ] `GET /subscriptions/{subscriptionId}/cidr` - Get CIDR allowlist
- [ ] `PUT /subscriptions/{subscriptionId}/cidr` - Update CIDR allowlist
- [ ] `GET /subscriptions/{subscriptionId}/pricing` - Get pricing info
- [ ] `GET /subscriptions/{subscriptionId}/maintenance-windows` - Get maintenance windows
- [ ] `POST /subscriptions/{subscriptionId}/maintenance-windows` - Create maintenance window
- [ ] `DELETE /subscriptions/{subscriptionId}/maintenance-windows` - Delete maintenance window

### Pro Databases (14 paths)
- [ ] `GET /subscriptions/{subscriptionId}/databases` - List databases
- [ ] `POST /subscriptions/{subscriptionId}/databases` - Create database
- [ ] `GET /subscriptions/{subscriptionId}/databases/{databaseId}` - Get database
- [ ] `PUT /subscriptions/{subscriptionId}/databases/{databaseId}` - Update database
- [ ] `DELETE /subscriptions/{subscriptionId}/databases/{databaseId}` - Delete database
- [ ] `POST /subscriptions/{subscriptionId}/databases/{databaseId}/backup` - Backup database
- [ ] `GET /subscriptions/{subscriptionId}/databases/{databaseId}/certificate` - Get TLS certificate
- [ ] `POST /subscriptions/{subscriptionId}/databases/{databaseId}/flush` - Flush database
- [ ] `POST /subscriptions/{subscriptionId}/databases/{databaseId}/import` - Import to database
- [ ] `PUT /subscriptions/{subscriptionId}/databases/{databaseId}/regions` - Update regions
- [ ] `GET /subscriptions/{subscriptionId}/databases/{databaseId}/slow-log` - Get slow log
- [ ] `GET /subscriptions/{subscriptionId}/databases/{databaseId}/tags` - List tags
- [ ] `PUT /subscriptions/{subscriptionId}/databases/{databaseId}/tags` - Update tags
- [ ] `DELETE /subscriptions/{subscriptionId}/databases/{databaseId}/tags/{tagKey}` - Delete tag
- [ ] `POST /subscriptions/{subscriptionId}/databases/{databaseId}/upgrade` - Upgrade database

### Connectivity - VPC Peering (4 paths)
- [ ] `GET /subscriptions/{subscriptionId}/peerings` - List peerings
- [ ] `POST /subscriptions/{subscriptionId}/peerings` - Create peering
- [ ] `GET /subscriptions/{subscriptionId}/peerings/{peeringId}` - Get peering
- [ ] `PUT /subscriptions/{subscriptionId}/peerings/{peeringId}` - Update peering
- [ ] `DELETE /subscriptions/{subscriptionId}/peerings/{peeringId}` - Delete peering

### Connectivity - Private Service Connect (4 paths)
- [ ] `GET /subscriptions/{subscriptionId}/private-service-connect` - List PSC endpoints
- [ ] `POST /subscriptions/{subscriptionId}/private-service-connect` - Create PSC endpoint
- [ ] `GET /subscriptions/{subscriptionId}/private-service-connect/{privateServiceConnectId}` - Get PSC endpoint
- [ ] `PUT /subscriptions/{subscriptionId}/private-service-connect/{privateServiceConnectId}` - Update PSC endpoint
- [ ] `DELETE /subscriptions/{subscriptionId}/private-service-connect/{privateServiceConnectId}` - Delete PSC endpoint

### Connectivity - Transit Gateway (5 paths)
- [ ] `GET /subscriptions/{subscriptionId}/transit-gateway` - List Transit Gateways
- [ ] `POST /subscriptions/{subscriptionId}/transit-gateway` - Create Transit Gateway
- [ ] `GET /subscriptions/{subscriptionId}/transit-gateway/{transitGatewayId}` - Get Transit Gateway
- [ ] `PUT /subscriptions/{subscriptionId}/transit-gateway/{transitGatewayId}` - Update Transit Gateway
- [ ] `DELETE /subscriptions/{subscriptionId}/transit-gateway/{transitGatewayId}` - Delete Transit Gateway

### Tasks & Logs (3 paths)
- [ ] `GET /tasks` - List tasks
- [ ] `GET /tasks/{taskId}` - Get task status
- [ ] `GET /logs` - Get system logs
- [ ] `GET /session-logs` - Get session logs

### Utility Endpoints (5 paths)
- [ ] `GET /data-persistence` - List data persistence options
- [ ] `GET /database-modules` - List database modules
- [ ] `GET /query-performance-factors` - List performance factors
- [ ] `GET /regions` - List regions
- [ ] `GET /throughput-measurement-by` - Get throughput measurements

## Component Schemas by Category

### User & Account Models (5 schemas)
- [ ] `AccountUser` - User account details
- [ ] `AccountUsers` - List of users
- [ ] `AccountUserOptions` - User options
- [ ] `AccountUserUpdateRequest` - Update user request
- [ ] `ACLUser` - ACL user details

### ACL Models (11 schemas)
- [ ] `AccountACLRedisRules` - List of Redis ACL rules
- [ ] `AccountACLRoles` - List of ACL roles
- [ ] `AccountACLUsers` - List of ACL users
- [ ] `AclRedisRuleCreateRequest` - Create Redis rule request
- [ ] `AclRedisRuleUpdateRequest` - Update Redis rule request
- [ ] `AclRoleCreateRequest` - Create role request
- [ ] `AclRoleDatabaseSpec` - Role database specification
- [ ] `AclRoleRedisRuleSpec` - Role Redis rule specification
- [ ] `AclRoleUpdateRequest` - Update role request
- [ ] `AclUserCreateRequest` - Create ACL user request
- [ ] `AclUserUpdateRequest` - Update ACL user request

### Subscription Models (28 schemas)
- [ ] `AccountSubscriptions` - List of subscriptions
- [ ] `BaseSubscriptionUpdateRequest` - Base subscription update
- [ ] `ProSubscriptionCreateRequest` - Create Pro subscription
- [ ] `ProSubscriptionUpdateRequest` - Update Pro subscription
- [ ] `ProSubscriptionRegions` - Pro subscription regions
- [ ] `FixedSubscriptionCreateRequest` - Create Fixed subscription
- [ ] `FixedSubscriptionDatabaseCreateRequest` - Create Fixed database
- [ ] `FixedSubscriptionPlans` - Fixed subscription plans
- [ ] `SubscriptionPricing` - Subscription pricing details
- [ ] `SubscriptionMaintenanceWindows` - Maintenance windows
- [ ] `SubscriptionRedisVersions` - Redis versions
- [ ] `ActiveActiveSubscriptionRegions` - Active-Active regions
- [ ] `ActiveActiveRegionCreateRequest` - Create AA region
- [ ] `ActiveActiveRegionDeleteRequest` - Delete AA region

### Database Models (19 schemas)
- [ ] `AccountSubscriptionDatabases` - List of databases
- [ ] `AccountFixedSubscriptionDatabases` - Fixed databases
- [ ] `DatabaseCreateRequest` - Create database request
- [ ] `DatabaseUpdateRequest` - Update database request
- [ ] `DatabaseBackupRequest` - Backup database request
- [ ] `DatabaseImportRequest` - Import database request
- [ ] `DatabaseFlushRequest` - Flush database request
- [ ] `DatabaseRegions` - Database regions
- [ ] `DatabaseSlowLog` - Slow log entries
- [ ] `DatabaseTags` - Database tags
- [ ] `CrdbFlushRequest` - CRDB flush request
- [ ] `CrdbRegionSpec` - CRDB region specification
- [ ] `CrdbUpdatePropertiesRequest` - CRDB update request
- [ ] `BdbVersionUpgradeStatus` - Version upgrade status

### Cloud Provider Models (5 schemas)
- [ ] `CloudAccount` - Cloud account details
- [ ] `CloudAccounts` - List of cloud accounts
- [ ] `CloudAccountCreateRequest` - Create cloud account
- [ ] `CloudAccountUpdateRequest` - Update cloud account
- [ ] `CloudTag` - Cloud resource tag
- [ ] `CloudTags` - List of cloud tags

### Connectivity Models (22 schemas)
- [ ] `VpcPeerings` - List of VPC peerings
- [ ] `VpcPeeringCreateRequest` - Create VPC peering
- [ ] `VpcPeeringUpdateRequest` - Update VPC peering
- [ ] `ActiveActiveVpcPeeringCreateAwsRequest` - AA AWS peering
- [ ] `ActiveActiveVpcPeeringCreateGcpRequest` - AA GCP peering
- [ ] `ActiveActiveVpcPeeringUpdateAwsRequest` - Update AA AWS peering
- [ ] `PrivateServiceConnect` - PSC endpoint details
- [ ] `PrivateServiceConnectCreateRequest` - Create PSC
- [ ] `PrivateServiceConnectUpdateRequest` - Update PSC
- [ ] `ActiveActivePscEndpointCreateRequest` - AA PSC create
- [ ] `ActiveActivePscEndpointUpdateRequest` - AA PSC update
- [ ] `TransitGateways` - List of Transit Gateways
- [ ] `TransitGatewayCreateRequest` - Create Transit Gateway
- [ ] `TransitGatewayUpdateRequest` - Update Transit Gateway
- [ ] `ActiveActiveTgwUpdateCidrsRequest` - Update AA TGW CIDRs
- [ ] `Cidr` - CIDR block
- [ ] `CidrWhiteListUpdateRequest` - Update CIDR allowlist

### Security Models (4 schemas)
- [ ] `CustomerManagedKey` - CMK details
- [ ] `CustomerManagedKeyAccessDetails` - CMK access details
- [ ] `TlsCertificate` - TLS certificate details
- [ ] `PasswordUpdateRequest` - Update password request

### Utility Models (16 schemas)
- [ ] `AccountSystemLogEntries` - System log entries
- [ ] `AccountSystemLogEntry` - Single system log entry
- [ ] `AccountSessionLogEntries` - Session log entries
- [ ] `AccountSessionLogEntry` - Single session log entry
- [ ] `Task` - Task details
- [ ] `TaskList` - List of tasks
- [ ] `DataPersistence` - Data persistence options
- [ ] `DatabaseModules` - Database module options
- [ ] `QueryPerformanceFactors` - Performance factors
- [ ] `Regions` - Available regions
- [ ] `ThroughputMeasurements` - Throughput measurements
- [ ] `PaymentMethods` - Payment methods list
- [ ] `ErrorResponse` - Error response format
- [ ] `Links` - HATEOAS links
- [ ] `Pagination` - Pagination metadata
- [ ] `EmptyResponse` - Empty success response

## Implementation Strategy

1. **Phase 1 - Core Models & Account** 
   - Implement common/utility schemas (ErrorResponse, Pagination, Links)
   - Implement Account handler (`GET /`)
   - Implement User management handlers

2. **Phase 2 - Subscriptions**
   - Implement Pro subscription handlers
   - Implement Fixed/Essentials subscription handlers
   - Related models and schemas

3. **Phase 3 - Databases**
   - Implement Pro database handlers
   - Implement Fixed database handlers
   - Database operations (backup, import, flush)

4. **Phase 4 - ACL & Security**
   - Implement ACL handlers (users, roles, rules)
   - TLS certificate management

5. **Phase 5 - Connectivity**
   - VPC Peering handlers
   - Private Service Connect handlers
   - Transit Gateway handlers

6. **Phase 6 - Monitoring & Utilities**
   - Tasks API
   - Logs (system and session)
   - Cloud accounts
   - Utility endpoints

## Notes

- All handlers should follow the pattern established in the backup directory
- Each handler module should include:
  - Handler struct with `new()` method
  - Request/Response models
  - Async methods matching API operations
  - Proper error handling using CloudError types
- Tests should be written for each handler using wiremock