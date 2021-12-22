import org.junit.platform.suite.api.IncludeClassNamePatterns
import org.junit.platform.suite.api.SelectPackages
import org.junit.platform.suite.api.Suite
import org.junit.platform.suite.api.SuiteDisplayName

@Suite
@SelectPackages("year2021")
@IncludeClassNamePatterns(".*")
@SuiteDisplayName("year2021")
class `2021Tests`