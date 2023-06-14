<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>gopay Payment Advice</name>
   <tag></tag>
   <elementGuidId>a9b4002f-eaa0-48d5-84df-e69738592330</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000\&quot;,\r\n    \&quot;accountType\&quot;: \&quot;10\&quot;,\r\n    \&quot;transactionAmount\&quot;: 20500,\r\n    \&quot;transDateTime\&quot;: \&quot;${transDateTime}\&quot;,\r\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\r\n    \&quot;localTime\&quot;: \&quot;${localTime}\&quot;,\r\n    \&quot;localDate\&quot;: \&quot;${localDate}\&quot;,\r\n    \&quot;settlementDate\&quot;: \&quot;${settlementDate}\&quot;,\r\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\r\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\r\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;referenceNumber\&quot;: \&quot;${referenceNumber}\&quot;,\r\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\r\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\r\n    \&quot;transactionData\&quot;: \&quot;${transactionData}\&quot;,\r\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\r\n    \&quot;nationalPmtData\&quot;: \&quot;${nationalPmtData}\&quot;,\r\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;transactionIndicator\&quot;: \&quot;4\&quot;,\r\n    \&quot;destinationInstCode\&quot;: \&quot;91400003500\&quot;\r\n}&quot;,
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
      <webElementGuid>c4b96b6e-ad66-4087-a4bc-1ad776a95f0f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-signature</name>
      <type>Main</type>
      <value>${SERVICE_SIGNATURE}</value>
      <webElementGuid>a3aacea3-2430-4e94-8f97-a15421bd9d56</webElementGuid>
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
      <id>bc393ab0-e3ae-46ed-ac41-9c4f3cd0f0cc</id>
      <masked>false</masked>
      <name>HOST</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>9e2503ad-3118-4aab-867f-dd92691b0118</id>
      <masked>false</masked>
      <name>TIMESTAMP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SERVICE_SIGNATURE</defaultValue>
      <description></description>
      <id>743ba137-b5b2-48a5-8da7-5d0ed6b30fd5</id>
      <masked>false</masked>
      <name>SERVICE_SIGNATURE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transDateTime</defaultValue>
      <description></description>
      <id>9d177274-ada3-42eb-b3b3-4657308b80e5</id>
      <masked>false</masked>
      <name>transDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localTime</defaultValue>
      <description></description>
      <id>220902e9-dd65-4861-b8b3-ec3ebd98d22b</id>
      <masked>false</masked>
      <name>localTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localDate</defaultValue>
      <description></description>
      <id>f39696cb-1662-4dda-844e-6fbbcc8de372</id>
      <masked>false</masked>
      <name>localDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.settlementDate</defaultValue>
      <description></description>
      <id>2366047f-b9ef-4f6f-91ee-3a899ddc12ac</id>
      <masked>false</masked>
      <name>settlementDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.referenceNumber</defaultValue>
      <description></description>
      <id>a67d5f9c-190e-4521-b4a5-3f25a68a27d7</id>
      <masked>false</masked>
      <name>referenceNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionData</defaultValue>
      <description></description>
      <id>02db27ff-07e4-4949-ad90-9b7d65cddd4e</id>
      <masked>false</masked>
      <name>transactionData</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.nationalPmtData</defaultValue>
      <description></description>
      <id>81ea323e-7d6b-40d5-9799-4459a52c04f6</id>
      <masked>false</masked>
      <name>nationalPmtData</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
