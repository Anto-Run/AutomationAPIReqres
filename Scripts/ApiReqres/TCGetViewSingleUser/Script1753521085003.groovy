import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import internal.GlobalVariable as GlobalVariable
import java.nio.file.Paths as Paths
import java.nio.file.Files as Files
import java.nio.charset.StandardCharsets as StandardCharsets
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration

// Load JSON Schema file
def schemaPath = Paths.get(RunConfiguration.getProjectDir(), 'Include/jsonSchema/responseGetViewSingleUser.json')
String jsonSchema = new String(Files.readAllBytes(schemaPath), StandardCharsets.UTF_8)

// Set request parameter
int id = 2

// Send request
ResponseObject response = WS.sendRequest(findTestObject(
	'ApiReqres/GetViewSingleUser', 
	[
		('baseUrl') : GlobalVariable.BASE_URL, 
		('xApiKey') : GlobalVariable.X_API_KEY, 
		('id') : id
	]
))



// Data Validate Response
String email = 'janet.weaver@reqres.in'

String first_name = 'Janet'

String last_name = 'Weaver'

String avatar = 'https://reqres.in/img/faces/2-image.jpg'

// Validate response
if ((response != null) && (response.getStatusCode() == 200)) {
    KeywordUtil.markPassed('Get View Single User Successfull!')

    WS.verifyElementPropertyValue(response, 'data.id', id)

    WS.verifyElementText(response, 'data.email', email)

    WS.verifyElementText(response, 'data.first_name', first_name)

    WS.verifyElementText(response, 'data.last_name', last_name)

    WS.verifyElementText(response, 'data.avatar', avatar)

    WS.validateJsonAgainstSchema(response, jsonSchema)
} else {
    KeywordUtil.markFailed('Get View Single User!')
}



