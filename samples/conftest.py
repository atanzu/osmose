import pytest

from fixtures.systest import OsmoseSystemTest

@pytest.fixture
def systest():
    return OsmoseSystemTest()
