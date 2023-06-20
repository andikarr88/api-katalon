import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import java.time.ZonedDateTime as ZonedDateTime
import java.time.ZoneOffset as ZoneOffset
import java.time.format.DateTimeFormatter as DateTimeFormatter

// Mendapatkan waktu saat ini dengan zona waktu yang diinginkan
ZonedDateTime currentTime = ZonedDateTime.now(ZoneOffset.ofHours(7))
// Format timestamp sesuai dengan kebutuhan
DateTimeFormatter formatter = DateTimeFormatter.ofPattern('yyyy-MM-dd\'T\'HH:mm:ssxxx')
String formattedTimestamp = currentTime.format(formatter)
// Tampilkan timestamp yang dihasilkan
println('Timestamp: ' + formattedTimestamp)
// Menyimpan formattedTimestamp ke GlobalVariable
GlobalVariable.TIMESTAMP = formattedTimestamp

signatureResponse = WS.sendRequest(findTestObject('Postman/auth/GET SIGNATURE'))
signature = WS.getElementPropertyValue(signatureResponse, 'signature')
println('signature is :' + signature)
GlobalVariable.AUTH_SIGNATURE = signature

tokenResponse = WS.sendRequest(findTestObject('Postman/auth/GET TOKEN', [('TIMESTAMP1') : GlobalVariable.TIMESTAMP, ('AUTH_SIGNATURE1') : GlobalVariable.AUTH_SIGNATURE]))
WS.verifyResponseStatusCode(tokenResponse, 200)
token = WS.getElementPropertyValue(tokenResponse, 'accessToken')
println('Token is : ' + token)
GlobalVariable.TOKEN = token
