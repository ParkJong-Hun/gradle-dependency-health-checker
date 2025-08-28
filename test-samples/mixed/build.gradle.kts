plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.android.library)
}

kotlin {
    androidTarget()
    ios()
    jvm()
    
    sourceSets {
        commonMain.dependencies {
            // Using libs references
            implementation(libs.kotlinx.coroutines.core)
            api(libs.ktor.client.core)
            
            // Using string dependencies
            implementation("com.google.code.gson:gson:2.10.1")
        }
        
        commonTest {
            dependencies {
                // Mix of libs and strings
                implementation(libs.kotlinx.coroutines.test)
                implementation("junit:junit:4.13.2")
                implementation("kotlin-test")
            }
        }
        
        androidMain {
            dependencies {
                // Using libs references
                implementation(libs.androidx.core.ktx)
                implementation(libs.ktor.client.android)
                
                // Using string dependencies  
                implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.7.0")
            }
        }
        
        iosMain.dependencies {
            // Mix of both
            implementation("io.ktor:ktor-client-darwin:2.3.5")
            implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.4.1")
        }
        
        jvmMain.dependencies {
            // Only string dependencies
            implementation("io.ktor:ktor-client-cio:2.3.5") 
            implementation("ch.qos.logback:logback-classic:1.4.11")
        }
    }
}

dependencies {
    // Regular dependencies block - mix of both
    implementation(libs.ktor.client.core)  // Duplicate with commonMain for testing
    implementation("com.squareup.retrofit2:retrofit:2.9.0")
}