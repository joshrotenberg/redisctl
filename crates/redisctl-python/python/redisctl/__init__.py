"""Redis Cloud and Enterprise management APIs for Python.

This package provides Python bindings for the redisctl Rust library,
enabling management of Redis Cloud and Redis Enterprise deployments.

Example:
    >>> from redisctl import CloudClient, EnterpriseClient
    >>>
    >>> # Redis Cloud
    >>> cloud = CloudClient(api_key="...", api_secret="...")
    >>> subs = cloud.subscriptions_sync()
    >>>
    >>> # Redis Enterprise
    >>> enterprise = EnterpriseClient(
    ...     base_url="https://cluster:9443",
    ...     username="admin",
    ...     password="secret"
    ... )
    >>> dbs = enterprise.databases_sync()
"""

# Import from the native Rust extension
from redisctl.redisctl import (
    CloudClient,
    EnterpriseClient,
    RedisCtlError,
    __version__,
)

__all__ = [
    "CloudClient",
    "EnterpriseClient",
    "RedisCtlError",
    "__version__",
]
