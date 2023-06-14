<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET SIGNATURE SERVICE -  - emoney gopay open denom payment</name>
   <tag></tag>
   <elementGuidId>390a36ca-7cc7-41e2-a127-b10a20fb7e03</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000\&quot;,\n    \&quot;accountType\&quot;: \&quot;20\&quot;,\n    \&quot;transactionAmount\&quot;: 20500,\n    \&quot;transDateTime\&quot;: \&quot;${GlobalVariable.transDateTime}\&quot;,\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\n    \&quot;localTime\&quot;: \&quot;${GlobalVariable.localTime}\&quot;,\n    \&quot;localDate\&quot;: \&quot;${GlobalVariable.localDate}\&quot;,\n    \&quot;settlementDate\&quot;: \&quot;${GlobalVariable.settlementDate}\&quot;,\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\n    \&quot;referenceNumber\&quot;: \&quot;${GlobalVariable.referenceNumber}\&quot;,\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\n    \&quot;transactionData\&quot;: \&quot;${GlobalVariable.transactionData}\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\n    \&quot;nationalPmtData\&quot;: \&quot;${GlobalVariable.nationalPmtData}\&quot;,\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\n    \&quot;transactionIndicator\&quot;: \&quot;2\&quot;,\n    \&quot;destinationInstCode\&quot;: \&quot;91400003500\&quot;\n}&quot;,
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
      <webElementGuid>11781939-ee2b-43b3-9ce9-4dd16bfc0e62</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1e714a50-2d21-4513-a26d-709ac322cf8c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${GlobalVariable.TIMESTAMP}</value>
      <webElementGuid>20e79d12-a340-411d-867f-faf866b7754f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-client-secret</name>
      <type>Main</type>
      <value>${GlobalVariable.X_CLIENT_SECRET}</value>
      <webElementGuid>34a90448-6fd4-4f38-b1a7-61d7a426e59e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>httpmethod</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>73d797b0-8a5e-4b02-993e-283411514bd9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>endpoinurl</name>
      <type>Main</type>
      <value>/jalin/openapi/payment/paymentcredit</value>
      <webElementGuid>cc086bc2-f943-473b-b49c-a532e4eaf051</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accesstoken</name>
      <type>Main</type>
      <value>${TOKEN}</value>
      <webElementGuid>cde1553e-a494-4786-b5da-e701ddc1ba70</webElementGuid>
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
