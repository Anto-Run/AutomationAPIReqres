import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.testobject.ResponseObject
import internal.GlobalVariable
import java.nio.file.Paths
import java.nio.file.Files
import java.nio.charset.StandardCharsets
import com.kms.katalon.core.configuration.RunConfiguration

// Load JSON Schema file
def schemaPath = Paths.get(RunConfiguration.getProjectDir(), 'Include/jsonSchema/responseGetListUser.json')
String jsonSchema = new String(Files.readAllBytes(schemaPath), StandardCharsets.UTF_8)

// Set request parameter
int page = 1

// Send request
ResponseObject response = WS.sendRequest(findTestObject(
	'ApiReqres/GetListUser',
	[
		('page')    : page,
		('baseUrl') : GlobalVariable.BASE_URL,
		('xApiKey') : GlobalVariable.X_API_KEY
	]
))

// Validate response
if (response != null && response.getStatusCode() == 200) {
	    KeywordUtil.markPassed('Get List User Successfull!')
	WS.verifyElementPropertyValue(response, 'page', page)
	WS.validateJsonAgainstSchema(response, jsonSchema)
} else {
	KeywordUtil.markFailed('Get List User Failed!')
}




