"""Tests for redisctl Python bindings.

These tests verify the basic functionality of the Python bindings without
requiring actual Redis Cloud or Enterprise instances.
"""

import os

import pytest


class TestModuleImports:
    """Test that the module imports correctly."""

    def test_import_module(self):
        """Test that we can import the redisctl module."""
        import redisctl

        assert redisctl is not None

    def test_version(self):
        """Test that version is available."""
        import redisctl

        assert hasattr(redisctl, "__version__")
        assert isinstance(redisctl.__version__, str)
        assert len(redisctl.__version__) > 0

    def test_classes_available(self):
        """Test that main classes are available."""
        from redisctl import CloudClient, EnterpriseClient, RedisCtlError

        assert CloudClient is not None
        assert EnterpriseClient is not None
        assert RedisCtlError is not None


class TestCloudClient:
    """Tests for CloudClient."""

    def test_constructor(self):
        """Test that we can create a CloudClient."""
        from redisctl import CloudClient

        client = CloudClient(
            api_key="test-key",
            api_secret="test-secret",
        )
        assert client is not None

    def test_constructor_with_options(self):
        """Test CloudClient with all options."""
        from redisctl import CloudClient

        client = CloudClient(
            api_key="test-key",
            api_secret="test-secret",
            base_url="https://api.example.com/v1",
            timeout_secs=60,
        )
        assert client is not None

    def test_from_env_missing_vars(self):
        """Test that from_env raises when env vars are missing."""
        from redisctl import CloudClient

        # Clear any existing env vars
        for var in [
            "REDIS_CLOUD_API_KEY",
            "REDIS_CLOUD_ACCOUNT_KEY",
            "REDIS_CLOUD_API_SECRET",
            "REDIS_CLOUD_SECRET_KEY",
            "REDIS_CLOUD_USER_KEY",
        ]:
            os.environ.pop(var, None)

        with pytest.raises(ValueError, match="API key not found"):
            CloudClient.from_env()

    def test_from_env_missing_secret(self):
        """Test that from_env raises when secret is missing."""
        from redisctl import CloudClient

        # Clear any existing env vars
        for var in [
            "REDIS_CLOUD_API_SECRET",
            "REDIS_CLOUD_SECRET_KEY",
            "REDIS_CLOUD_USER_KEY",
        ]:
            os.environ.pop(var, None)

        os.environ["REDIS_CLOUD_API_KEY"] = "test-key"

        try:
            with pytest.raises(ValueError, match="API secret not found"):
                CloudClient.from_env()
        finally:
            os.environ.pop("REDIS_CLOUD_API_KEY", None)

    def test_from_env_success(self):
        """Test that from_env works with valid env vars."""
        from redisctl import CloudClient

        os.environ["REDIS_CLOUD_API_KEY"] = "test-key"
        os.environ["REDIS_CLOUD_API_SECRET"] = "test-secret"

        try:
            client = CloudClient.from_env()
            assert client is not None
        finally:
            os.environ.pop("REDIS_CLOUD_API_KEY", None)
            os.environ.pop("REDIS_CLOUD_API_SECRET", None)

    def test_from_env_alternate_vars(self):
        """Test that from_env works with alternate env var names."""
        from redisctl import CloudClient

        os.environ["REDIS_CLOUD_ACCOUNT_KEY"] = "test-key"
        os.environ["REDIS_CLOUD_USER_KEY"] = "test-secret"

        try:
            client = CloudClient.from_env()
            assert client is not None
        finally:
            os.environ.pop("REDIS_CLOUD_ACCOUNT_KEY", None)
            os.environ.pop("REDIS_CLOUD_USER_KEY", None)

    def test_has_sync_methods(self):
        """Test that sync methods exist."""
        from redisctl import CloudClient

        client = CloudClient(api_key="test", api_secret="test")

        assert hasattr(client, "subscriptions_sync")
        assert hasattr(client, "subscription_sync")
        assert hasattr(client, "databases_sync")
        assert hasattr(client, "database_sync")
        assert hasattr(client, "get_sync")
        assert hasattr(client, "post_sync")
        assert hasattr(client, "put_sync")
        assert hasattr(client, "delete_sync")

    def test_has_async_methods(self):
        """Test that async methods exist."""
        from redisctl import CloudClient

        client = CloudClient(api_key="test", api_secret="test")

        assert hasattr(client, "subscriptions")
        assert hasattr(client, "subscription")
        assert hasattr(client, "databases")
        assert hasattr(client, "database")
        assert hasattr(client, "get")
        assert hasattr(client, "post")
        assert hasattr(client, "put")
        assert hasattr(client, "delete")


class TestEnterpriseClient:
    """Tests for EnterpriseClient."""

    def test_constructor(self):
        """Test that we can create an EnterpriseClient."""
        from redisctl import EnterpriseClient

        client = EnterpriseClient(
            base_url="https://cluster:9443",
            username="admin@example.com",
            password="password",
        )
        assert client is not None

    def test_constructor_with_options(self):
        """Test EnterpriseClient with all options."""
        from redisctl import EnterpriseClient

        client = EnterpriseClient(
            base_url="https://cluster:9443",
            username="admin@example.com",
            password="password",
            insecure=True,
            timeout_secs=60,
        )
        assert client is not None

    def test_from_env_missing_vars(self):
        """Test that from_env raises when env vars are missing."""
        from redisctl import EnterpriseClient

        # Clear any existing env vars
        for var in [
            "REDIS_ENTERPRISE_URL",
            "REDIS_ENTERPRISE_USER",
            "REDIS_ENTERPRISE_PASSWORD",
        ]:
            os.environ.pop(var, None)

        with pytest.raises(Exception):
            EnterpriseClient.from_env()

    def test_from_env_success(self):
        """Test that from_env works with valid env vars."""
        from redisctl import EnterpriseClient

        os.environ["REDIS_ENTERPRISE_URL"] = "https://cluster:9443"
        os.environ["REDIS_ENTERPRISE_USER"] = "admin@example.com"
        os.environ["REDIS_ENTERPRISE_PASSWORD"] = "password"

        try:
            client = EnterpriseClient.from_env()
            assert client is not None
        finally:
            os.environ.pop("REDIS_ENTERPRISE_URL", None)
            os.environ.pop("REDIS_ENTERPRISE_USER", None)
            os.environ.pop("REDIS_ENTERPRISE_PASSWORD", None)

    def test_has_sync_methods(self):
        """Test that sync methods exist."""
        from redisctl import EnterpriseClient

        client = EnterpriseClient(
            base_url="https://cluster:9443",
            username="admin",
            password="pass",
        )

        # Cluster methods
        assert hasattr(client, "cluster_info_sync")
        assert hasattr(client, "cluster_stats_sync")
        assert hasattr(client, "license_sync")

        # Database methods
        assert hasattr(client, "databases_sync")
        assert hasattr(client, "database_sync")
        assert hasattr(client, "database_stats_sync")

        # Node methods
        assert hasattr(client, "nodes_sync")
        assert hasattr(client, "node_sync")
        assert hasattr(client, "node_stats_sync")

        # User methods
        assert hasattr(client, "users_sync")
        assert hasattr(client, "user_sync")

        # Raw API methods
        assert hasattr(client, "get_sync")
        assert hasattr(client, "post_sync")
        assert hasattr(client, "put_sync")
        assert hasattr(client, "delete_sync")

    def test_has_async_methods(self):
        """Test that async methods exist."""
        from redisctl import EnterpriseClient

        client = EnterpriseClient(
            base_url="https://cluster:9443",
            username="admin",
            password="pass",
        )

        # Cluster methods
        assert hasattr(client, "cluster_info")
        assert hasattr(client, "cluster_stats")
        assert hasattr(client, "license")

        # Database methods
        assert hasattr(client, "databases")
        assert hasattr(client, "database")
        assert hasattr(client, "database_stats")

        # Node methods
        assert hasattr(client, "nodes")
        assert hasattr(client, "node")
        assert hasattr(client, "node_stats")

        # User methods
        assert hasattr(client, "users")
        assert hasattr(client, "user")

        # Raw API methods
        assert hasattr(client, "get")
        assert hasattr(client, "post")
        assert hasattr(client, "put")
        assert hasattr(client, "delete")
