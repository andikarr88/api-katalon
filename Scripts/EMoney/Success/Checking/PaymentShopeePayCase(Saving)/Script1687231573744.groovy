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
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import java.time.ZonedDateTime as ZonedDateTime
import java.time.ZoneOffset as ZoneOffset
import java.time.format.DateTimeFormatter as DateTimeFormatter

// Mendapatkan waktu saat ini dengan zona waktu yang diinginkan
ZonedDateTime currentTime = ZonedDateTime.now(ZoneOffset.ofHours(7))

// Format timestamp sesuai dengan kebutuhan
DateTimeFormatter formatter = DateTimeFormatter.ofPattern('MMdd')

String formattedLocalDate = currentTime.format(formatter)

// Tampilkan timestamp yang dihasilkan
println('Local Date: ' + formattedLocalDate)

// Menyimpan formattedTimestamp ke GlobalVariable
GlobalVariable.localDate = formattedLocalDate

// Mendapatkan waktu saat ini dengan zona waktu yang diinginkan
ZonedDateTime currentTime1 = ZonedDateTime.now(ZoneOffset.ofHours(7))

// Format timestamp sesuai dengan kebutuhan
DateTimeFormatter formatter1 = DateTimeFormatter.ofPattern('HHmmss')

String formattedLocalTime = currentTime1.format(formatter1)

// Tampilkan timestamp yang dihasilkan
println('Local Time: ' + formattedLocalTime)

// Menyimpan formattedTimestamp ke GlobalVariable
GlobalVariable.localTime = formattedLocalTime

String formattedTransDateTime = formattedLocalDate + formattedLocalTime

// Tampilkan timestamp yang dihasilkan
println('Trans Date Time: ' + formattedTransDateTime)

// Menyimpan formattedTimestamp ke GlobalVariable
GlobalVariable.transDateTime = formattedTransDateTime

// Mendapatkan waktu saat ini dengan zona waktu yang diinginkan
ZonedDateTime currentTime2 = ZonedDateTime.now(ZoneOffset.ofHours(7))

// Format timestamp sesuai dengan kebutuhan
DateTimeFormatter formatter2 = DateTimeFormatter.ofPattern('MMdd')

String formattedSettlementDate = currentTime2.plusDays(1).format(formatter2)

// Tampilkan timestamp yang dihasilkan
println('SettlementDate: ' + formattedSettlementDate)

// Menyimpan formattedTimestamp ke GlobalVariable
GlobalVariable.settlementDate = formattedSettlementDate

// Mendapatkan waktu saat ini dengan zona waktu yang diinginkan
ZonedDateTime currentTime3 = ZonedDateTime.now(ZoneOffset.ofHours(7))

// Format timestamp sesuai dengan kebutuhan
DateTimeFormatter formatter3 = DateTimeFormatter.ofPattern('yyMMddHHmmss')

String formattedReferenceNumber = currentTime3.format(formatter3)

// Tampilkan timestamp yang dihasilkan
println('Reference Number: ' + formattedReferenceNumber)

// Menyimpan formattedTimestamp ke GlobalVariable
GlobalVariable.referenceNumber = formattedReferenceNumber

signatureResponse = WS.sendRequest(findTestObject('Postman/emoney/opendenom Success/Checking/GET SIGNATURE SERVICE -  - emoney shopeepay open denom payment'))

signature = WS.getElementPropertyValue(signatureResponse, 'signature')

println('signature service is :' + signature)

GlobalVariable.SERVICE_SIGNATURE = signature

danaPaymentResponse = WS.sendRequest(findTestObject('Postman/emoney/opendenom Success/Checking/shopeepay payment'))

RC = WS.getElementPropertyValue(danaPaymentResponse, 'responseCode')

String responseBody = danaPaymentResponse.getResponseBodyContent()

// Menampilkan response body di console
println('Response Body:')

println(responseBody)

println('RC is : ' + RC)

WS.verifyResponseStatusCode(danaPaymentResponse, 200)

WS.verifyElementPropertyValue(danaPaymentResponse, 'responseCode', '00')

