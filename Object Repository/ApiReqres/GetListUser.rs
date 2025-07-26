<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Api Get List User</description>
   <name>GetListUser</name>
   <tag></tag>
   <elementGuidId>63d71380-e6a1-4ccc-aaec-b4c9bac466f7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-API-KEY</name>
      <type>Main</type>
      <value>${xApiKey}</value>
      <webElementGuid>18e24d30-f315-4aca-934e-721d43fb10c3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/api/users?page=${page}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>fa4f9ac7-1b55-442c-8203-116b5ca61b43</id>
      <name>validateJsonSchemaResponse</name>
      <type>AUTO_DETECT</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Include/jsonSchema/responseGetListUser.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>0749aca9-1ddf-4a63-84a6-bc0bcbac5b74</id>
      <masked>false</masked>
      <name>page</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>f369f674-8578-47d5-91b3-1cbe09fcd4c8</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.X_API_KEY</defaultValue>
      <description></description>
      <id>e41b52ef-6dd8-4677-b8cb-f5c06e36f0d1</id>
      <masked>false</masked>
      <name>xApiKey</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




//Verification
if(response != null &amp;&amp; response.getStatusCode().equals(200)) {
	
	KeywordUtil.markPassed(&quot;Hit API Get List successfull!&quot;)
	
	
}else {
	KeywordUtil.markFailed(&quot;Hit API Get List User Failed!&quot;)
}
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
