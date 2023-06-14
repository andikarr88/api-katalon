<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET SIGNATURE</name>
   <tag></tag>
   <elementGuidId>60478439-a01e-48d0-994e-65e3e5bf1455</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
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
      <webElementGuid>140601b0-5500-4868-9e03-7f1c86c4ca89</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-timestamp</name>
      <type>Main</type>
      <value>${TIMESTAMP1}</value>
      <webElementGuid>bc0c6946-ef0b-48db-8c7a-0df615613856</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-client-key</name>
      <type>Main</type>
      <value>3b775b73a2cd31d1b2a3ae52aea02c18</value>
      <webElementGuid>53a92b09-cdd5-4571-89bf-e7c718a249e9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-private-key</name>
      <type>Main</type>
      <value>MIIBVAIBADANBgkqhkiG9w0BAQEFAASCAT4wggE6AgEAAkEAvp7LRAa0eU0cJC6PHSFWAGFOr5qPjuCwxfLmMoXaa+AbFTjVNur0zRrvNjegxX+QmyP0xehVNrlO+lnZq/ramQIDAQABAkA+24g1v2xFz7qm57+DoJmGeJAE8hfCyq8gJ0/neyIijeq2NKc93+H3mh8HBt2DPuwmgyq/pZ8HpK1X5uyjFCU9AiEA8eEar/2bKhP0UfTgsqae8KXLeYElcEHX5S1YcP9fZK8CIQDJv58kHa+OLGvknNmMMJ4I6D2KgSikVaD44Sz+v7eXNwIgZM4M4kXOUeYJD9L/hlT8roxaVaQmJze5s3CHiGhVqE8CIBEr2BWw2SJWsZAxsWp3MNw9OA+z0ou6JgtIzxWXp76dAiEAucfV79IKlHPW1ZmkwtZlwTLLvk9Oj/PIPZwyX+NcKBQ=</value>
      <webElementGuid>f43a1790-691c-4350-a9b4-e5c7de8632b0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.132.128.4:8088/auth/signature</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TIMESTAMP</defaultValue>
      <description></description>
      <id>74363bae-0bc1-4b45-953c-f7108c678831</id>
      <masked>false</masked>
      <name>TIMESTAMP1</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.MethodParameterObject
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.configuration.RunConfiguration

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import java.text.SimpleDateFormat
import java.util.TimeZone
import java.util.Calendar
import java.time.LocalDate
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import java.time.ZonedDateTime
import java.time.ZoneOffset


RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// Mendapatkan waktu saat ini dengan zona waktu yang diinginkan
ZonedDateTime currentTime = ZonedDateTime.now(ZoneOffset.ofHours(7))

// Format timestamp sesuai dengan kebutuhan
DateTimeFormatter formatter = DateTimeFormatter.ofPattern(&quot;yyyy-MM-dd'T'HH:mm:ssxxx&quot;)
String formattedTimestamp = currentTime.format(formatter)

// Tampilkan timestamp yang dihasilkan
println(&quot;Timestamp: &quot; + formattedTimestamp)

// Simpan formattedTimestamp ke Global Variable TIMESTAMP
GlobalVariable.TIMESTAMP = formattedTimestamp</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
