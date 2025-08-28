plugins {
    kotlin("multiplatform")
    id("com.android.library")
}

kotlin {
    androidTarget()
    ios()
    jvm()
    
    sourceSets {
        commonMain.dependencies {
            implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")
            api("io.ktor:ktor-client-core:2.3.5")
        }
        
        commonTest {
            dependencies {
                implementation("kotlin-test")
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.7.3")
            }
        }
        
        androidMain {
            dependencies {
                implementation("androidx.core:core-ktx:1.13.0")
                implementation("io.ktor:ktor-client-android:2.3.5")
            }
        }
        
        iosMain.dependencies {
            implementation("io.ktor:ktor-client-darwin:2.3.5")
        }
        
        jvmMain.dependencies {
            implementation("io.ktor:ktor-client-cio:2.3.5")
        }
    }
}

dependencies {
    implementation("com.google.code.gson:gson:2.10.1")
}