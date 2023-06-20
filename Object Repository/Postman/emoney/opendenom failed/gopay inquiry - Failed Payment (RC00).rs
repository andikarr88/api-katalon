<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>gopay inquiry - Failed Payment (RC00)</name>
   <tag></tag>
   <elementGuidId>604cea5d-dc34-476a-849a-e726612c000f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.TOKEN}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000001\&quot;,\n    \&quot;accountType\&quot;: \&quot;10\&quot;,\n    \&quot;transactionAmount\&quot;: 20000,\n    \&quot;transDateTime\&quot;: \&quot;${GlobalVariable.transDateTime}\&quot;,\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\n    \&quot;localTime\&quot;: \&quot;${GlobalVariable.localTime}\&quot;,\n    \&quot;localDate\&quot;: \&quot;${GlobalVariable.localDate}\&quot;,\n    \&quot;settlementDate\&quot;: \&quot;${GlobalVariable.settlementDate}\&quot;,\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\n    \&quot;referenceNumber\&quot;: \&quot;${GlobalVariable.referenceNumber}\&quot;,\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\n    \&quot;transactionData\&quot;: \&quot;PI0501000CN24             08123459009PM0222\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\n    \&quot;nationalPmtData\&quot;: \&quot;\&quot;,\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\n    \&quot;transactionIndicator\&quot;: \&quot;0\&quot;,\n    \&quot;destinationInstCode\&quot;: \&quot;91400003500\&quot;\n}&quot;,
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
      <webElementGuid>aaa735f1-a9bd-433c-8f42-c525234cd55b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${GlobalVariable.TIMESTAMP}</value>
      <webElementGuid>c6609256-05e7-4143-9e3d-4296de8b11cb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-signature</name>
      <type>Main</type>
      <value>${GlobalVariable.SERVICE_SIGNATURE}</value>
      <webElementGuid>717de5a5-d1e1-446c-8ef6-e0711e270398</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TOKEN}</value>
      <webElementGuid>3b13c7c4-293b-4a2a-ab21-73f4e42f2875</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.HOST}/jalin/openapi/payment/paymentinquiry</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.HOST</defaultValue>
      <description></description>
      <id>c5b9cf59-a2c8-4951-ad8c-58490576f368</id>
      <masked>false</masked>
      <name>HOST</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>3292ad61-23f4-402f-8868-8a6cfcb112e6</id>
      <masked>false</masked>
      <name>TIMESTAMP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SERVICE_SIGNATURE</defaultValue>
      <description></description>
      <id>3c558d5e-ab96-4892-b2ba-81eeac2a4e80</id>
      <masked>false</masked>
      <name>SERVICE_SIGNATURE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transDateTime</defaultValue>
      <description></description>
      <id>d958f4b0-a7a2-4c0f-ba69-97c184269f9d</id>
      <masked>false</masked>
      <name>transDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localTime</defaultValue>
      <description></description>
      <id>74030362-afb5-48c6-afb1-2e19d0718aa7</id>
      <masked>false</masked>
      <name>localTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localDate</defaultValue>
      <description></description>
      <id>f3c8b5ec-a796-44b8-b399-924062cdbe4a</id>
      <masked>false</masked>
      <name>localDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.settlementDate</defaultValue>
      <description></description>
      <id>e0bfff30-da54-4bac-a112-f4a52b4b39a1</id>
      <masked>false</masked>
      <name>settlementDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.referenceNumber</defaultValue>
      <description></description>
      <id>3077321a-5ea4-418a-a865-9f2e7768198d</id>
      <masked>false</masked>
      <name>referenceNumber</name>
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
