<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Api Update User</description>
   <name>UpdateUser</name>
   <tag></tag>
   <elementGuidId>b4821f1d-c8a7-4fa1-98ff-5ed529ed981d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${nameUser}\&quot;,\n    \&quot;job\&quot;: \&quot;${jobUser}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>45d9eab2-a7fe-4b3f-a75e-b7a0552f3e92</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-API-KEY</name>
      <type>Main</type>
      <value>${xApiKey}</value>
      <webElementGuid>2e34e0b8-f85c-47fc-91ee-f673453508ec</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${baseUrl}/api/users/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>5f5df55a-08cc-4294-8300-ded25be79b30</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.X_API_KEY</defaultValue>
      <description></description>
      <id>24c99993-dead-4c6a-a96b-f7fa32f9dbca</id>
      <masked>false</masked>
      <name>xApiKey</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>91c1de38-c008-47bc-a1f9-2fbcaf210e0f</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'Runanto'</defaultValue>
      <description></description>
      <id>5f09adb7-b591-4f0b-83ba-52c23b17add1</id>
      <masked>false</masked>
      <name>nameUser</name>
   </variables>
   <variables>
      <defaultValue>'QA/QC'</defaultValue>
      <description></description>
      <id>6690dbb1-34e2-495a-a14e-97f4e3d32117</id>
      <masked>false</masked>
      <name>jobUser</name>
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
	
	KeywordUtil.markPassed(&quot;Hit API Update User successfull!&quot;)
	
}else {
	KeywordUtil.markFailed(&quot;Hit API Update User Failed!&quot;)
}
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
