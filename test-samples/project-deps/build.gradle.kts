dependencies {
    // External dependencies (should be analyzed)
    implementation("com.google.code.gson:gson:2.10.1")
    api("io.ktor:ktor-client-core:2.3.5")
    
    // Project dependencies (should be ignored)
    implementation(project(":core"))
    api(project(":shared"))
    testImplementation(project(":test-utils"))
    
    // Projects accessor (should be ignored)
    implementation(projects.data.database)
    api(projects.ui.components)
    
    // Mixed case
    implementation("androidx.core:core-ktx:1.13.0")
    implementation(project(":another-module"))
}