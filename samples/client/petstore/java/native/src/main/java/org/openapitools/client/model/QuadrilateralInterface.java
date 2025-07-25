/*
 * OpenAPI Petstore
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package org.openapitools.client.model;

import org.apache.commons.lang3.builder.EqualsBuilder;
import org.apache.commons.lang3.builder.HashCodeBuilder;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.StringJoiner;
import java.util.Objects;
import java.util.Map;
import java.util.HashMap;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonTypeName;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Arrays;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import org.openapitools.client.ApiClient;
/**
 * QuadrilateralInterface
 */
@JsonPropertyOrder({
  QuadrilateralInterface.JSON_PROPERTY_QUADRILATERAL_TYPE
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
public class QuadrilateralInterface {
  public static final String JSON_PROPERTY_QUADRILATERAL_TYPE = "quadrilateralType";
  @javax.annotation.Nonnull
  private String quadrilateralType;

  public QuadrilateralInterface() { 
  }

  public QuadrilateralInterface quadrilateralType(@javax.annotation.Nonnull String quadrilateralType) {
    this.quadrilateralType = quadrilateralType;
    return this;
  }

  /**
   * Get quadrilateralType
   * @return quadrilateralType
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_QUADRILATERAL_TYPE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getQuadrilateralType() {
    return quadrilateralType;
  }


  @JsonProperty(JSON_PROPERTY_QUADRILATERAL_TYPE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setQuadrilateralType(@javax.annotation.Nonnull String quadrilateralType) {
    this.quadrilateralType = quadrilateralType;
  }


  /**
   * Return true if this QuadrilateralInterface object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    return EqualsBuilder.reflectionEquals(this, o, false, null, true);
  }

  @Override
  public int hashCode() {
    return HashCodeBuilder.reflectionHashCode(this);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class QuadrilateralInterface {\n");
    sb.append("    quadrilateralType: ").append(toIndentedString(quadrilateralType)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }

  /**
   * Convert the instance into URL query string.
   *
   * @return URL query string
   */
  public String toUrlQueryString() {
    return toUrlQueryString(null);
  }

  /**
   * Convert the instance into URL query string.
   *
   * @param prefix prefix of the query string
   * @return URL query string
   */
  public String toUrlQueryString(String prefix) {
    String suffix = "";
    String containerSuffix = "";
    String containerPrefix = "";
    if (prefix == null) {
      // style=form, explode=true, e.g. /pet?name=cat&type=manx
      prefix = "";
    } else {
      // deepObject style e.g. /pet?id[name]=cat&id[type]=manx
      prefix = prefix + "[";
      suffix = "]";
      containerSuffix = "]";
      containerPrefix = "[";
    }

    StringJoiner joiner = new StringJoiner("&");

    // add `quadrilateralType` to the URL query string
    if (getQuadrilateralType() != null) {
      joiner.add(String.format("%squadrilateralType%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getQuadrilateralType()))));
    }

    return joiner.toString();
  }

    public static class Builder {

    private QuadrilateralInterface instance;

    public Builder() {
      this(new QuadrilateralInterface());
    }

    protected Builder(QuadrilateralInterface instance) {
      this.instance = instance;
    }

    public QuadrilateralInterface.Builder quadrilateralType(String quadrilateralType) {
      this.instance.quadrilateralType = quadrilateralType;
      return this;
    }


    /**
    * returns a built QuadrilateralInterface instance.
    *
    * The builder is not reusable.
    */
    public QuadrilateralInterface build() {
      try {
        return this.instance;
      } finally {
        // ensure that this.instance is not reused
        this.instance = null;
      }
    }

    @Override
    public String toString() {
      return getClass() + "=(" + instance + ")";
    }
  }

  /**
  * Create a builder with no initialized field.
  */
  public static QuadrilateralInterface.Builder builder() {
    return new QuadrilateralInterface.Builder();
  }

  /**
  * Create a builder with a shallow copy of this instance.
  */
  public QuadrilateralInterface.Builder toBuilder() {
    return new QuadrilateralInterface.Builder()
      .quadrilateralType(getQuadrilateralType());
  }

}

