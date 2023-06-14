<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET SIGNATURE SERVICE -  emoney gopay open denom inquiry</name>
   <tag></tag>
   <elementGuidId>d478e84a-d2e6-4ab7-9105-e6083aa43300</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000001\&quot;,\n    \&quot;accountType\&quot;: \&quot;20\&quot;,\n    \&quot;transactionAmount\&quot;: 20000,\n    \&quot;transDateTime\&quot;: \&quot;${GlobalVariable.transDateTime}\&quot;,\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\n    \&quot;localTime\&quot;: \&quot;${GlobalVariable.localTime}\&quot;,\n    \&quot;localDate\&quot;: \&quot;${GlobalVariable.localDate}\&quot;,\n    \&quot;settlementDate\&quot;: \&quot;${GlobalVariable.settlementDate}\&quot;,\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\n    \&quot;referenceNumber\&quot;: \&quot;${GlobalVariable.referenceNumber}\&quot;,\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\n    \&quot;transactionData\&quot;: \&quot;PI0502000CN24             08123450000PM0222\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\n    \&quot;nationalPmtData\&quot;: \&quot;\&quot;,\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\n    \&quot;transactionIndicator\&quot;: \&quot;0\&quot;,\n    \&quot;destinationInstCode\&quot;: \&quot;91400003500\&quot;\n}&quot;,
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
      <webElementGuid>f9b5972a-4945-4256-ae0a-305174b8b4fc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>933b58e6-f061-4755-bb38-7a1c67de1280</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${GlobalVariable.TIMESTAMP}</value>
      <webElementGuid>2fca64d9-4485-465a-a116-80912e7cc78b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-client-secret</name>
      <type>Main</type>
      <value>${GlobalVariable.X_CLIENT_SECRET}</value>
      <webElementGuid>f4585a22-9697-4051-95d3-e6b40dedabbd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>httpmethod</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>96d85ae6-f523-4199-ad70-fc67c071ab60</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>endpoinurl</name>
      <type>Main</type>
      <value>/jalin/openapi/payment/paymentinquiry</value>
      <webElementGuid>2a3e722b-eff6-41f3-8179-4778bc8be96f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accesstoken</name>
      <type>Main</type>
      <value>${TOKEN}</value>
      <webElementGuid>f57906c8-3d5a-4a9b-9ba7-373b56d75251</webElementGuid>
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
