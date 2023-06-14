<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET SIGNATURE SERVICE -  emoney gopay open denom Payment - Failed</name>
   <tag></tag>
   <elementGuidId>e1dc4644-6a87-4003-978f-b8fcb199c59a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000\&quot;,\r\n    \&quot;accountType\&quot;: \&quot;10\&quot;,\r\n    \&quot;transactionAmount\&quot;: 20500,\r\n    \&quot;transDateTime\&quot;: \&quot;${transDateTime}\&quot;,\r\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\r\n    \&quot;localTime\&quot;: \&quot;${localTime}\&quot;,\r\n    \&quot;localDate\&quot;: \&quot;${localDate}\&quot;,\r\n    \&quot;settlementDate\&quot;: \&quot;${settlementDate}\&quot;,\r\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\r\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\r\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;referenceNumber\&quot;: \&quot;${referenceNumber}\&quot;,\r\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\r\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\r\n    \&quot;transactionData\&quot;: \&quot;${transactionData}\&quot;,\r\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\r\n    \&quot;nationalPmtData\&quot;: \&quot;${nationalPmtData}\&quot;,\r\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;transactionIndicator\&quot;: \&quot;2\&quot;,\r\n    \&quot;destinationInstCode\&quot;: \&quot;91400003500\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e4f6fad0-fb8b-4d7d-aef6-5f5da896a085</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${TIMESTAMP}</value>
      <webElementGuid>e0aa7139-6bec-40b1-b0fc-0ff6b5c19729</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-client-secret</name>
      <type>Main</type>
      <value>${X_CLIENT_SECRET}</value>
      <webElementGuid>47c9e47a-9139-478c-90b3-55f28091c754</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>httpmethod</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>b1ef713c-666a-4397-812b-6f775050cce4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>endpoinurl</name>
      <type>Main</type>
      <value>/jalin/openapi/payment/paymentcredit</value>
      <webElementGuid>30d9fa1e-4049-40e3-b459-ef8746fd8719</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accesstoken</name>
      <type>Main</type>
      <value>${TOKEN}</value>
      <webElementGuid>d3d39ce9-6ccf-4e1f-abf3-0d90e3de6cb8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HOST}/auth/signature/service</restUrl>
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
      <id>b78c14e0-36c8-436e-adcb-b1fc0b36ee62</id>
      <masked>false</masked>
      <name>HOST</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>2bbd6e86-d1d8-4f20-bed6-7152feedbfa8</id>
      <masked>false</masked>
      <name>TIMESTAMP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.X_CLIENT_SECRET</defaultValue>
      <description></description>
      <id>9a4b0072-ae31-4e97-a23a-e0073fad7d98</id>
      <masked>false</masked>
      <name>X_CLIENT_SECRET</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TOKEN</defaultValue>
      <description></description>
      <id>26487a63-3bed-49ef-b822-85992003cdb4</id>
      <masked>false</masked>
      <name>TOKEN</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transDateTime</defaultValue>
      <description></description>
      <id>9faab6c2-9dbf-4fce-9059-ad17188c8fab</id>
      <masked>false</masked>
      <name>transDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localTime</defaultValue>
      <description></description>
      <id>86396f99-d683-4de5-bc09-8e8895cbcee1</id>
      <masked>false</masked>
      <name>localTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localDate</defaultValue>
      <description></description>
      <id>a6012c04-c6ea-434d-a73a-b4840fc24a90</id>
      <masked>false</masked>
      <name>localDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.settlementDate</defaultValue>
      <description></description>
      <id>471a4dd1-e0ce-4363-8137-0a6a27a0c43e</id>
      <masked>false</masked>
      <name>settlementDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.referenceNumber</defaultValue>
      <description></description>
      <id>23bb8c03-0fac-4a8d-8558-5985f8038e11</id>
      <masked>false</masked>
      <name>referenceNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionData</defaultValue>
      <description></description>
      <id>2b2eeaab-e3b6-4efb-9e0c-ea70457fb87d</id>
      <masked>false</masked>
      <name>transactionData</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.nationalPmtData</defaultValue>
      <description></description>
      <id>d109ae30-2b2a-4875-af09-8998b94bc10e</id>
      <masked>false</masked>
      <name>nationalPmtData</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>