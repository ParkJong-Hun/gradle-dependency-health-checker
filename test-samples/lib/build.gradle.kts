plugins {
    id("java-library")
    kotlin("jvm")
}

dependencies {
    api("androidx.core:core-ktx:1.13.0")
    implementation("com.google.code.gson:gson:2.10.1")
    implementation("com.squareup.retrofit2:retrofit:2.9.0")
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.mockito:mockito-core:5.6.0")
}