<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>lat_lon_code</name>
   <tag></tag>
   <elementGuidId>1342b92b-ddba-4cbd-a631-e57efcc05ca2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url_geo}/direct?q=${country}&amp;appid=${appid}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>5bd02c60-6945-4127-88f0-9191a16ecc86</id>
      <name>test</name>
      <type>AUTO_DETECT</type>
      <dataType>URL</dataType>
      <target>REQUEST</target>
      <data></data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.url_geo</defaultValue>
      <description></description>
      <id>adea4dae-ef70-43cb-b4c8-74183193997e</id>
      <masked>false</masked>
      <name>url_geo</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.country</defaultValue>
      <description></description>
      <id>deedd1a2-400e-469b-8b59-8afcc21a857f</id>
      <masked>false</masked>
      <name>country</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.appid</defaultValue>
      <description></description>
      <id>2c9e737f-9b3b-4ca8-9d9e-11b8fd6816f9</id>
      <masked>false</masked>
      <name>appid</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
