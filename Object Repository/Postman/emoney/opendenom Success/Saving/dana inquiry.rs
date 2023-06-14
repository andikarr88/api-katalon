<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>dana inquiry</name>
   <tag></tag>
   <elementGuidId>67491c61-2160-4e6a-a202-58444cddedff</elementGuidId>
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
  &quot;text&quot;: &quot;{\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000\&quot;,\n    \&quot;accountType\&quot;: \&quot;10\&quot;,\n    \&quot;transactionAmount\&quot;: 10000,\n    \&quot;transDateTime\&quot;: \&quot;${GlobalVariable.transDateTime}\&quot;,\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\n    \&quot;localTime\&quot;: \&quot;${GlobalVariable.localTime}\&quot;,\n    \&quot;localDate\&quot;: \&quot;${GlobalVariable.localDate}\&quot;,\n    \&quot;settlementDate\&quot;: \&quot;${GlobalVariable.settlementDate}\&quot;,\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\n    \&quot;referenceNumber\&quot;: \&quot;${GlobalVariable.referenceNumber}\&quot;,\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\n    \&quot;transactionData\&quot;: \&quot;PI0501000CN24              0817345545PM0222\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\n    \&quot;nationalPmtData\&quot;: \&quot;\&quot;,\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\n    \&quot;transactionIndicator\&quot;: \&quot;0\&quot;,\n    \&quot;destinationInstCode\&quot;: \&quot;91500003500\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>1407a4a1-d281-41e2-ae53-b658d2ab9729</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${GlobalVariable.TIMESTAMP}</value>
      <webElementGuid>22515053-1aaf-41b8-9cf6-f8eeb07d06ed</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-signature</name>
      <type>Main</type>
      <value>${GlobalVariable.SERVICE_SIGNATURE}</value>
      <webElementGuid>6cb1ae1b-a1ae-4f32-ad7f-a15b73ef1318</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TOKEN}</value>
      <webElementGuid>97d7f7e8-ba34-4f60-9d61-8bfc6dea7ef9</webElementGuid>
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
