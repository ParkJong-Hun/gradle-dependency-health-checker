kotlin {
    sourceSets {
        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(libs.kotlinxSerializationJson)
            api(libs.kotlinxDatetime)
            api(libs.kotlinxCollectionsImmutable)
            api(libs.soilQueryCore)
        }

        androidMain.dependencies {
            implementation(libs.androidxAppCompat)
        }
    }
}

dependencies {
    commonMainImplementation(libs.material3)
}