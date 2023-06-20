<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET SIGNATURE SERVICE - Gopay Advice Payment</name>
   <tag></tag>
   <elementGuidId>7eb27dec-edf0-4f32-b2dd-fd1e99693acd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000\&quot;,\n    \&quot;accountType\&quot;: \&quot;20\&quot;,\n    \&quot;transactionAmount\&quot;: 20000,\n    \&quot;transDateTime\&quot;: \&quot;${GlobalVariable.transDateTime}\&quot;,\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\n    \&quot;localTime\&quot;: \&quot;${GlobalVariable.localTime}\&quot;,\n    \&quot;localDate\&quot;: \&quot;${GlobalVariable.localDate}\&quot;,\n    \&quot;settlementDate\&quot;: \&quot;${GlobalVariable.settlementDate}\&quot;,\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\n    \&quot;referenceNumber\&quot;: \&quot;${GlobalVariable.referenceNumber}\&quot;,\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\n    \&quot;transactionData\&quot;: \&quot;${GlobalVariable.transactionData}\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\n    \&quot;nationalPmtData\&quot;: \&quot;${GlobalVariable.nationalPmtData}\&quot;,\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\n    \&quot;transactionIndicator\&quot;: \&quot;4\&quot;,\n    \&quot;destinationInstCode\&quot;: \&quot;91400003500\&quot;\n}&quot;,
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
      <webElementGuid>0b5b3a4e-7773-4f8b-9f3e-fccd5570778f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2841705f-371d-4f8c-9075-7550ac30eef0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${GlobalVariable.TIMESTAMP}</value>
      <webElementGuid>f3fd8959-4bf3-41ab-b06b-5e092f0aa1b8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-client-secret</name>
      <type>Main</type>
      <value>4wjywZj8WVOtBXY3yEq8kHudvCxjNp07FhvMcxMpd59P1FKayOBouMqlalwzmr/7LkwPZMSUfoB9iYbde4RMAw==</value>
      <webElementGuid>867196f1-d618-4b6c-b0a3-1818509f505e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>httpmethod</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>d5f2c41f-1152-4cd7-8bd8-b8e03dedd286</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>endpoinurl</name>
      <type>Main</type>
      <value>/jalin/openapi/payment/advice</value>
      <webElementGuid>e701aebf-5beb-47e2-ab77-23705108ddd4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accesstoken</name>
      <type>Main</type>
      <value>${GlobalVariable.TOKEN}</value>
      <webElementGuid>6493d049-42d6-49f1-ab15-453ead4c99c8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.HOST}/auth/signature/service</restUrl>
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
