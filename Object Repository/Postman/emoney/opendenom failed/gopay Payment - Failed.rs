<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>gopay Payment - Failed</name>
   <tag></tag>
   <elementGuidId>a4ec4984-757d-4d02-87b2-0fa4f150cbdc</elementGuidId>
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
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${TIMESTAMP}</value>
      <webElementGuid>adc0b3de-3289-4c41-8880-a3fe5bca9f23</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-signature</name>
      <type>Main</type>
      <value>${SERVICE_SIGNATURE}</value>
      <webElementGuid>8772838d-a8b0-4593-94b6-a765078a853f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${HOST}/jalin/openapi/payment/paymentcredit</restUrl>
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
      <id>cdf70215-5edd-4acd-a994-5e539d38239f</id>
      <masked>false</masked>
      <name>HOST</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>a5689fa1-a892-474d-b747-967d223eaecb</id>
      <masked>false</masked>
      <name>TIMESTAMP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SERVICE_SIGNATURE</defaultValue>
      <description></description>
      <id>21538d2f-425a-4ebd-a60e-b246179e6481</id>
      <masked>false</masked>
      <name>SERVICE_SIGNATURE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transDateTime</defaultValue>
      <description></description>
      <id>81b7a01a-c074-4acf-a36a-4ef7ffceb7e0</id>
      <masked>false</masked>
      <name>transDateTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localTime</defaultValue>
      <description></description>
      <id>e32c8811-c101-448b-be77-e22442773726</id>
      <masked>false</masked>
      <name>localTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.localDate</defaultValue>
      <description></description>
      <id>a5b085c9-3041-4b80-a0b6-888bf8e5b840</id>
      <masked>false</masked>
      <name>localDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.settlementDate</defaultValue>
      <description></description>
      <id>a29dcb6f-17bd-4847-b16d-dfc0e99b8634</id>
      <masked>false</masked>
      <name>settlementDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.referenceNumber</defaultValue>
      <description></description>
      <id>d05eb6d6-6f79-4607-ac5b-1886e70286bb</id>
      <masked>false</masked>
      <name>referenceNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionData</defaultValue>
      <description></description>
      <id>154ad33e-cccd-459b-8d22-fa7a50dd6506</id>
      <masked>false</masked>
      <name>transactionData</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.nationalPmtData</defaultValue>
      <description></description>
      <id>6d11d414-b1ed-4f97-946a-4f10691883b3</id>
      <masked>false</masked>
      <name>nationalPmtData</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
