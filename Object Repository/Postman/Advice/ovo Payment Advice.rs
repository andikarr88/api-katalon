<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ovo Payment Advice</name>
   <tag></tag>
   <elementGuidId>9614c125-9946-4e36-9c90-e8e1bc96e352</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000\&quot;,\r\n    \&quot;accountType\&quot;: \&quot;10\&quot;,\r\n    \&quot;transactionAmount\&quot;: 20500,\r\n    \&quot;transDateTime\&quot;: \&quot;${transDateTime}\&quot;,\r\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\r\n    \&quot;localTime\&quot;: \&quot;${localTime}\&quot;,\r\n    \&quot;localDate\&quot;: \&quot;${localDate}\&quot;,\r\n    \&quot;settlementDate\&quot;: \&quot;${settlementDate}\&quot;,\r\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\r\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\r\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;referenceNumber\&quot;: \&quot;${referenceNumber}\&quot;,\r\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\r\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\r\n    \&quot;transactionData\&quot;: \&quot;${transactionData}\&quot;,\r\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\r\n    \&quot;nationalPmtData\&quot;: \&quot;${nationalPmtData}\&quot;,\r\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;transactionIndicator\&quot;: \&quot;4\&quot;,\r\n    \&quot;destinationInstCode\&quot;: \&quot;91200003500\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${TIMESTAMP}</value>
      <webElementGuid>4d747898-cbdc-4a4b-b33d-d61b602e4ad2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-signature</name>
      <type>Main</type>
      <value>${SERVICE_SIGNATURE}</value>
      <webElementGuid>e009e589-27f1-4831-a53c-586863e433cb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HOST}/jalin/openapi/payment/advice</restUrl>
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
      <id>fcb4c5a2-2740-4226-9a95-ee13023f121d</id>
      <masked>false</masked>
      <name>HOST</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>7d0d88f7-c64b-417f-8e96-6642b34ed95b</id>
      <masked>false</masked>
      <name>TIMESTAMP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SERVICE_SIGNATURE</defaultValue>
      <description></description>
      <id>b38e6746-210d-4730-b51f-038ba75727df</id>
      <masked>false</masked>
      <name>SERVICE_SIGNATURE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transDateTime</defaultValue>
      <description></description>
      <id>17e0ee9e-8e41-489b-8fee-f794d57ef738</id>
      <masked>false</masked>
      <name>transDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localTime</defaultValue>
      <description></description>
      <id>5c54b89e-ec04-4480-b9d7-b3cb4d6fa061</id>
      <masked>false</masked>
      <name>localTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localDate</defaultValue>
      <description></description>
      <id>4ba3d3b4-00b2-45b3-b50b-0a2a3ec8d986</id>
      <masked>false</masked>
      <name>localDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.settlementDate</defaultValue>
      <description></description>
      <id>1f400bf2-74bd-4219-a963-19d5d1a7ccfb</id>
      <masked>false</masked>
      <name>settlementDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.referenceNumber</defaultValue>
      <description></description>
      <id>38bc032b-9e46-4723-b4f4-fa2db5bb88e6</id>
      <masked>false</masked>
      <name>referenceNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionData</defaultValue>
      <description></description>
      <id>96645184-c6df-496f-9701-d16383cae45a</id>
      <masked>false</masked>
      <name>transactionData</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.nationalPmtData</defaultValue>
      <description></description>
      <id>a7714294-61be-4397-9900-d23b47ccf2dc</id>
      <masked>false</masked>
      <name>nationalPmtData</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
