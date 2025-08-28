dependencies {
    // Duplicate with mixed/build.gradle.kts commonMain and main dependencies
    implementation(libs.ktor.client.core)
    api("com.google.code.gson:gson:2.10.1")  
}