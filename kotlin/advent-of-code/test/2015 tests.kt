import org.junit.platform.suite.api.IncludeClassNamePatterns
import org.junit.platform.suite.api.SelectPackages
import org.junit.platform.suite.api.Suite
import org.junit.platform.suite.api.SuiteDisplayName

@Suite
@SelectPackages("year2015")
@IncludeClassNamePatterns(".*")
@SuiteDisplayName("year2015")
class `2015 tests`