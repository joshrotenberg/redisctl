"""Type stubs for redisctl Python bindings."""

from typing import Any, Awaitable, Optional

__version__: str

class RedisCtlError(Exception):
    """Base exception for redisctl errors."""
    ...

class CloudClient:
    """Redis Cloud API client.

    Provides access to Redis Cloud management APIs for subscriptions,
    databases, and other cloud resources.

    Example:
        ```python
        client = CloudClient(api_key="...", api_secret="...")

        # Async
        subs = await client.subscriptions()

        # Sync
        subs = client.subscriptions_sync()
        ```
    """

    def __init__(
        self,
        api_key: str,
        api_secret: str,
        base_url: Optional[str] = None,
        timeout_secs: Optional[int] = None,
    ) -> None:
        """Create a new Redis Cloud client.

        Args:
            api_key: Redis Cloud API key
            api_secret: Redis Cloud API secret
            base_url: Optional base URL (default: https://api.redislabs.com/v1)
            timeout_secs: Optional timeout in seconds (default: 30)
        """
        ...

    # Subscriptions
    def subscriptions(self) -> Awaitable[list[dict[str, Any]]]:
        """List all subscriptions (async)."""
        ...

    def subscriptions_sync(self) -> list[dict[str, Any]]:
        """List all subscriptions (sync)."""
        ...

    def subscription(self, subscription_id: int) -> Awaitable[dict[str, Any]]:
        """Get a specific subscription by ID (async)."""
        ...

    def subscription_sync(self, subscription_id: int) -> dict[str, Any]:
        """Get a specific subscription by ID (sync)."""
        ...

    # Databases
    def databases(
        self,
        subscription_id: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> Awaitable[list[dict[str, Any]]]:
        """List databases in a subscription (async)."""
        ...

    def databases_sync(
        self,
        subscription_id: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> list[dict[str, Any]]:
        """List databases in a subscription (sync)."""
        ...

    def database(
        self,
        subscription_id: int,
        database_id: int,
    ) -> Awaitable[dict[str, Any]]:
        """Get a specific database by ID (async)."""
        ...

    def database_sync(
        self,
        subscription_id: int,
        database_id: int,
    ) -> dict[str, Any]:
        """Get a specific database by ID (sync)."""
        ...

    # Raw API
    def get(self, path: str) -> Awaitable[dict[str, Any]]:
        """Execute a raw GET request (async)."""
        ...

    def get_sync(self, path: str) -> dict[str, Any]:
        """Execute a raw GET request (sync)."""
        ...

    def post(self, path: str, body: dict[str, Any]) -> Awaitable[dict[str, Any]]:
        """Execute a raw POST request (async)."""
        ...

    def post_sync(self, path: str, body: dict[str, Any]) -> dict[str, Any]:
        """Execute a raw POST request (sync)."""
        ...

    def put(self, path: str, body: dict[str, Any]) -> Awaitable[dict[str, Any]]:
        """Execute a raw PUT request (async)."""
        ...

    def put_sync(self, path: str, body: dict[str, Any]) -> dict[str, Any]:
        """Execute a raw PUT request (sync)."""
        ...

    def delete(self, path: str) -> Awaitable[dict[str, Any]]:
        """Execute a raw DELETE request (async)."""
        ...

    def delete_sync(self, path: str) -> dict[str, Any]:
        """Execute a raw DELETE request (sync)."""
        ...


class EnterpriseClient:
    """Redis Enterprise API client.

    Provides access to Redis Enterprise cluster management APIs for databases,
    nodes, users, and cluster operations.

    Example:
        ```python
        client = EnterpriseClient(
            base_url="https://cluster:9443",
            username="admin@redis.local",
            password="secret",
            insecure=True
        )

        # Async
        dbs = await client.databases()

        # Sync
        dbs = client.databases_sync()
        ```
    """

    def __init__(
        self,
        base_url: str,
        username: str,
        password: str,
        insecure: bool = False,
        timeout_secs: Optional[int] = None,
    ) -> None:
        """Create a new Redis Enterprise client.

        Args:
            base_url: Cluster URL (e.g., "https://cluster:9443")
            username: Username for authentication
            password: Password for authentication
            insecure: Allow insecure TLS (self-signed certs), default False
            timeout_secs: Optional timeout in seconds (default: 30)
        """
        ...

    @staticmethod
    def from_env() -> "EnterpriseClient":
        """Create client from environment variables.

        Reads:
            - REDIS_ENTERPRISE_URL
            - REDIS_ENTERPRISE_USER
            - REDIS_ENTERPRISE_PASSWORD
            - REDIS_ENTERPRISE_INSECURE
        """
        ...

    # Cluster
    def cluster_info(self) -> Awaitable[dict[str, Any]]:
        """Get cluster information (async)."""
        ...

    def cluster_info_sync(self) -> dict[str, Any]:
        """Get cluster information (sync)."""
        ...

    def cluster_stats(self) -> Awaitable[dict[str, Any]]:
        """Get cluster statistics (async)."""
        ...

    def cluster_stats_sync(self) -> dict[str, Any]:
        """Get cluster statistics (sync)."""
        ...

    def license(self) -> Awaitable[dict[str, Any]]:
        """Get license information (async)."""
        ...

    def license_sync(self) -> dict[str, Any]:
        """Get license information (sync)."""
        ...

    # Databases
    def databases(self) -> Awaitable[list[dict[str, Any]]]:
        """List all databases (async)."""
        ...

    def databases_sync(self) -> list[dict[str, Any]]:
        """List all databases (sync)."""
        ...

    def database(self, uid: int) -> Awaitable[dict[str, Any]]:
        """Get a specific database by UID (async)."""
        ...

    def database_sync(self, uid: int) -> dict[str, Any]:
        """Get a specific database by UID (sync)."""
        ...

    def database_stats(self, uid: int) -> Awaitable[dict[str, Any]]:
        """Get database statistics (async)."""
        ...

    def database_stats_sync(self, uid: int) -> dict[str, Any]:
        """Get database statistics (sync)."""
        ...

    # Nodes
    def nodes(self) -> Awaitable[list[dict[str, Any]]]:
        """List all nodes (async)."""
        ...

    def nodes_sync(self) -> list[dict[str, Any]]:
        """List all nodes (sync)."""
        ...

    def node(self, uid: int) -> Awaitable[dict[str, Any]]:
        """Get a specific node by UID (async)."""
        ...

    def node_sync(self, uid: int) -> dict[str, Any]:
        """Get a specific node by UID (sync)."""
        ...

    def node_stats(self, uid: int) -> Awaitable[dict[str, Any]]:
        """Get node statistics (async)."""
        ...

    def node_stats_sync(self, uid: int) -> dict[str, Any]:
        """Get node statistics (sync)."""
        ...

    # Users
    def users(self) -> Awaitable[list[dict[str, Any]]]:
        """List all users (async)."""
        ...

    def users_sync(self) -> list[dict[str, Any]]:
        """List all users (sync)."""
        ...

    def user(self, uid: int) -> Awaitable[dict[str, Any]]:
        """Get a specific user by UID (async)."""
        ...

    def user_sync(self, uid: int) -> dict[str, Any]:
        """Get a specific user by UID (sync)."""
        ...

    # Raw API
    def get(self, path: str) -> Awaitable[dict[str, Any]]:
        """Execute a raw GET request (async)."""
        ...

    def get_sync(self, path: str) -> dict[str, Any]:
        """Execute a raw GET request (sync)."""
        ...

    def post(self, path: str, body: dict[str, Any]) -> Awaitable[dict[str, Any]]:
        """Execute a raw POST request (async)."""
        ...

    def post_sync(self, path: str, body: dict[str, Any]) -> dict[str, Any]:
        """Execute a raw POST request (sync)."""
        ...

    def put(self, path: str, body: dict[str, Any]) -> Awaitable[dict[str, Any]]:
        """Execute a raw PUT request (async)."""
        ...

    def put_sync(self, path: str, body: dict[str, Any]) -> dict[str, Any]:
        """Execute a raw PUT request (sync)."""
        ...

    def delete(self, path: str) -> Awaitable[dict[str, Any]]:
        """Execute a raw DELETE request (async)."""
        ...

    def delete_sync(self, path: str) -> dict[str, Any]:
        """Execute a raw DELETE request (sync)."""
        ...
