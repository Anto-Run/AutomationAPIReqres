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
def schemaPath = Paths.get(RunConfiguration.getProjectDir(), 'Include/jsonSchema/responseRegisterSuccessfull.json')

String jsonSchema = new String(Files.readAllBytes(schemaPath), StandardCharsets.UTF_8)

// Set request parameter
int id = 4

// Send request get user
ResponseObject response = WS.sendRequest(findTestObject('ApiReqres/GetViewSingleUser', [('baseUrl') : GlobalVariable.BASE_URL
            , ('xApiKey') : GlobalVariable.X_API_KEY, ('id') : id]))

//Get data Email From Response Get Single User and set email
def slurper = new groovy.json.JsonSlurper()
def result = slurper.parseText(response.getResponseBodyContent())
def email = result.data.email;
String password ='pistol'

KeywordUtil.logInfo(email)

response = WS.sendRequest(findTestObject('ApiReqres/RegisterSuccessfull', [('baseUrl') : GlobalVariable.BASE_URL, ('xApiKey') : GlobalVariable.X_API_KEY
            , ('email') : email, ('password') : password]))


// Validate response
if ((response != null) && (response.getStatusCode() == 200)) {
    KeywordUtil.markPassed('Register Successfull - Successfull!')
    WS.verifyElementPropertyValue(response, 'id', id)
	WS.validateJsonAgainstSchema(response, jsonSchema)
	
} else {
    KeywordUtil.markFailed('Register Successfull - Failed!')
}

