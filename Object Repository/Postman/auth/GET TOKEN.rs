<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET TOKEN</name>
   <tag></tag>
   <elementGuidId>ba57dcb8-2cae-4e3b-a067-ca309674b394</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;granttype\&quot;: \&quot;client_credentials\&quot;,\n    \&quot;additionalInfo\&quot;: {}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f435c0db-6486-47ab-b89e-882e0b0dd0f9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${TIMESTAMP1}</value>
      <webElementGuid>9d344e81-67dd-4c6f-9b34-1ad8ea0a6281</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-client-key</name>
      <type>Main</type>
      <value>3b775b73a2cd31d1b2a3ae52aea02c18</value>
      <webElementGuid>17f66bef-9800-4172-b012-16f027701fae</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-signature</name>
      <type>Main</type>
      <value>${AUTH_SIGNATURE1}</value>
      <webElementGuid>aab41645-0a50-4ced-b17d-2c4af5b1a47d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.132.128.4:8088/auth/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>b41054d8-cb4b-4a63-8579-1c50142e0796</id>
      <masked>false</masked>
      <name>TIMESTAMP1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.AUTH_SIGNATURE</defaultValue>
      <description></description>
      <id>18d29142-ecd8-4197-8c42-d47e19934fa3</id>
      <masked>false</masked>
      <name>AUTH_SIGNATURE1</name>
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
