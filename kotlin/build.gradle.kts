plugins {
    kotlin("jvm") version "1.6.10"
}

group = "neyb.adventofcode"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

subprojects {
    apply(plugin = "org.jetbrains.kotlin.jvm")

    repositories {
        mavenCentral()
    }

    dependencies {
        if (name != "puzzle-utils") implementation(project(":puzzle-utils"))
        //        implementation("org.jetbrains.kotlinx:kotlinx-collections-immutable:0.3.4")

        //        testImplementation(kotlin("test"))
        testImplementation("ch.tutteli.atrium:atrium-fluent-en_GB:0.17.0")
        //        testImplementation("io.strikt:strikt-core:0.33.0")
        //        testImplementation ("org.junit.jupiter:junit-jupiter-api:5.8.1")

        testImplementation("org.junit.jupiter:junit-jupiter-api:5.8.2")
        testImplementation("org.junit.platform:junit-platform-suite-api:1.8.2")

        testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.8.2")
        testRuntimeOnly("org.junit.platform:junit-platform-suite-engine:1.8.2")
    }

    tasks.test {
        useJUnitPlatform()
    }

    tasks.compileKotlin {
        kotlinOptions.jvmTarget = "16"
    }


    configure<org.jetbrains.kotlin.gradle.dsl.KotlinJvmProjectExtension> {
        sourceSets["main"].kotlin.srcDirs("src")
        sourceSets["test"].kotlin.srcDirs("test")
    }
}