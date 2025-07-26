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
def schemaPath = Paths.get(RunConfiguration.getProjectDir(), 'Include/jsonSchema/responseUpdateUser.json')

String jsonSchema = new String(Files.readAllBytes(schemaPath), StandardCharsets.UTF_8)

// Set request parameter
int id = 2

String name = 'Runanto'

String job = 'Software Quality Assurance'

// Send request
ResponseObject response = WS.sendRequest(findTestObject(
	'ApiReqres/UpdateUser', 
	[
		('baseUrl') : GlobalVariable.BASE_URL, 
		('xApiKey') : GlobalVariable.X_API_KEY, 
		('id') : 2, ('nameUser') : name, 
		('jobUser') : job
	]
))



// Validate response
if ((response != null) && (response.getStatusCode() == 200)) {
    KeywordUtil.markPassed('Update User Successfull!')

    WS.verifyElementText(response, 'name', name)

    WS.verifyElementText(response, 'job', job)

    WS.validateJsonAgainstSchema(response, jsonSchema)
} else {
    KeywordUtil.markFailed('Update User User!')
}