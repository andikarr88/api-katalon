<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET SIGNATURE SERVICE -  emoney link open denom inquiry Wrong ID Pelanggan</name>
   <tag></tag>
   <elementGuidId>b16bbfee-0518-45e4-8106-b96444f11cba</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;cardNumber\&quot;: \&quot;0000000000000000\&quot;,\r\n    \&quot;accountType\&quot;: \&quot;10\&quot;,\r\n    \&quot;transactionAmount\&quot;: 20000,\r\n    \&quot;transDateTime\&quot;: \&quot;${transDateTime}\&quot;,\r\n    \&quot;traceNumber\&quot;: \&quot;000001\&quot;,\r\n    \&quot;localTime\&quot;: \&quot;${localTime}\&quot;,\r\n    \&quot;localDate\&quot;: \&quot;${localDate}\&quot;,\r\n    \&quot;settlementDate\&quot;: \&quot;${settlementDate}\&quot;,\r\n    \&quot;channelCode\&quot;: \&quot;6017\&quot;,\r\n    \&quot;posEntryMode\&quot;: \&quot;999\&quot;,\r\n    \&quot;acquirerID\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;referenceNumber\&quot;: \&quot;${referenceNumber}\&quot;,\r\n    \&quot;terminalID\&quot;: \&quot;0000000000000003\&quot;,\r\n    \&quot;terminalNameLoc\&quot;: \&quot;TEST MENARA DEA       JAKARTA SELAT068ID\&quot;,\r\n    \&quot;transactionData\&quot;: \&quot;PI0501000CN24            082333222111PM0222\&quot;,\r\n    \&quot;currencyCode\&quot;: \&quot;360\&quot;,\r\n    \&quot;nationalPmtData\&quot;: \&quot;\&quot;,\r\n    \&quot;issuerCode\&quot;: \&quot;00000000111\&quot;,\r\n    \&quot;transactionIndicator\&quot;: \&quot;0\&quot;,\r\n    \&quot;destinationInstCode\&quot;: \&quot;91100003500\&quot;\r\n}&quot;,
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
      <webElementGuid>6812cb4a-8f7a-416e-be13-6092222b0446</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${TIMESTAMP}</value>
      <webElementGuid>fa218b75-b6d4-499f-871b-9ddf74aad887</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-client-secret</name>
      <type>Main</type>
      <value>${X_CLIENT_SECRET}</value>
      <webElementGuid>c5f9995b-d9d7-476f-b0d7-a1190077ef25</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>httpmethod</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>6f3120bc-35f5-4481-a78f-809df38839a9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>endpoinurl</name>
      <type>Main</type>
      <value>/jalin/openapi/payment/paymentinquiry</value>
      <webElementGuid>9cb46fe2-6e00-4126-aa29-dfb1d2b2c591</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accesstoken</name>
      <type>Main</type>
      <value>${TOKEN}</value>
      <webElementGuid>b4d97955-c771-4a52-86bb-c5980e08105c</webElementGuid>
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
      <id>a4403907-f993-4a11-8f73-0806c46ce5f5</id>
      <masked>false</masked>
      <name>HOST</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>111f76e5-55da-46f3-9d91-602815e4dc25</id>
      <masked>false</masked>
      <name>TIMESTAMP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.X_CLIENT_SECRET</defaultValue>
      <description></description>
      <id>d28c2a3c-f762-41ac-b9f7-e5590756d342</id>
      <masked>false</masked>
      <name>X_CLIENT_SECRET</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TOKEN</defaultValue>
      <description></description>
      <id>1937fd7f-75e1-4c00-9992-f30163e8e464</id>
      <masked>false</masked>
      <name>TOKEN</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transDateTime</defaultValue>
      <description></description>
      <id>df1148bc-708c-4c68-96db-b01c49a37914</id>
      <masked>false</masked>
      <name>transDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localTime</defaultValue>
      <description></description>
      <id>766c181f-b7e0-497d-b44c-eb9178acc682</id>
      <masked>false</masked>
      <name>localTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localDate</defaultValue>
      <description></description>
      <id>f585db25-4474-4c25-b878-e742a016692c</id>
      <masked>false</masked>
      <name>localDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.settlementDate</defaultValue>
      <description></description>
      <id>ce0753a6-320a-4530-aefd-aba1d16127de</id>
      <masked>false</masked>
      <name>settlementDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.referenceNumber</defaultValue>
      <description></description>
      <id>d8bd9f63-8f6d-42c3-b72b-9a22451f9ed6</id>
      <masked>false</masked>
      <name>referenceNumber</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
